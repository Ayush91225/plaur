use plaur_common::{
    auth::AuthService as CommonAuthService,
    cache::CacheService,
    database::DbPool,
    error::{PlaurdError, Result},
    models::user::{CreateUserRequest, LoginRequest, LoginResponse, User, UserProfile, UserRole},
    utils::generate_id,
};
use sqlx::Row;
use std::time::Duration;
use tracing::{error, info};
use uuid::Uuid;

pub use plaur_common::auth::AuthService;

pub struct UserService<'a> {
    db_pool: &'a DbPool,
    cache: &'a CacheService,
    auth_service: &'a AuthService,
}

impl<'a> UserService<'a> {
    pub fn new(db_pool: &'a DbPool, cache: &'a CacheService, auth_service: &'a AuthService) -> Self {
        Self {
            db_pool,
            cache,
            auth_service,
        }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserProfile> {
        // Check if user already exists
        if self.user_exists_by_email(&request.email).await? {
            return Err(PlaurdError::Conflict {
                message: "User with this email already exists".to_string(),
            });
        }

        let user_id = generate_id();
        let password_hash = AuthService::hash_password(&request.password)?;

        let query = r#"
            INSERT INTO users (id, email, password_hash, first_name, last_name, role, is_active, email_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, true, false, NOW(), NOW())
            RETURNING id, email, first_name, last_name, role
        "#;

        let row = sqlx::query(query)
            .bind(user_id)
            .bind(&request.email)
            .bind(&password_hash)
            .bind(&request.first_name)
            .bind(&request.last_name)
            .bind(&request.role)
            .fetch_one(self.db_pool)
            .await?;

        Ok(UserProfile {
            id: row.get("id"),
            email: row.get("email"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            role: row.get("role"),
        })
    }

    pub async fn authenticate_user(&self, request: LoginRequest) -> Result<LoginResponse> {
        let user = self.get_user_by_email(&request.email).await?;

        if !AuthService::verify_password(&request.password, &user.password_hash)? {
            return Err(PlaurdError::Authentication {
                message: "Invalid credentials".to_string(),
            });
        }

        if !user.is_active {
            return Err(PlaurdError::Authentication {
                message: "Account is deactivated".to_string(),
            });
        }

        let access_token = self.auth_service.generate_token(user.id, &user.email, user.role.clone())?;
        let refresh_token = format!("refresh_{}", generate_id());

        // Store refresh token in cache
        self.cache
            .set(&refresh_token, &user.id, Some(Duration::from_secs(30 * 24 * 3600)))
            .await?;

        // Update last login
        self.update_last_login(user.id).await?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            user: UserProfile {
                id: user.id,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                role: user.role,
            },
        })
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<LoginResponse> {
        let user_id: Option<Uuid> = self.cache.get(refresh_token).await?;
        
        let user_id = user_id.ok_or_else(|| PlaurdError::Authentication {
            message: "Invalid refresh token".to_string(),
        })?;

        let user = self.get_user_by_id(user_id).await?;
        
        let access_token = self.auth_service.generate_token(user.id, &user.email, user.role.clone())?;
        let new_refresh_token = format!("refresh_{}", generate_id());

        // Replace old refresh token
        self.cache.delete(refresh_token).await?;
        self.cache
            .set(&new_refresh_token, &user.id, Some(Duration::from_secs(30 * 24 * 3600)))
            .await?;

        Ok(LoginResponse {
            access_token,
            refresh_token: new_refresh_token,
            user: UserProfile {
                id: user.id,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                role: user.role,
            },
        })
    }

    pub async fn logout(&self, token: &str) -> Result<()> {
        // Add token to blacklist
        let claims = self.auth_service.verify_token(token)?;
        let blacklist_key = format!("blacklist_{}", token);
        let ttl = Duration::from_secs((claims.exp as u64).saturating_sub(chrono::Utc::now().timestamp() as u64));
        
        self.cache.set(&blacklist_key, &true, Some(ttl)).await?;
        Ok(())
    }

    pub async fn verify_token(&self, token: &str) -> Result<UserProfile> {
        // Check if token is blacklisted
        let blacklist_key = format!("blacklist_{}", token);
        if self.cache.exists(&blacklist_key).await? {
            return Err(PlaurdError::Authentication {
                message: "Token has been revoked".to_string(),
            });
        }

        let claims = self.auth_service.verify_token(token)?;
        let user = self.get_user_by_id(claims.sub).await?;

        Ok(UserProfile {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
        })
    }

    async fn user_exists_by_email(&self, email: &str) -> Result<bool> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(self.db_pool)
            .await?;
        Ok(count > 0)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(self.db_pool)
            .await?
            .ok_or_else(|| PlaurdError::NotFound {
                resource: "User".to_string(),
            })
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(self.db_pool)
            .await?
            .ok_or_else(|| PlaurdError::NotFound {
                resource: "User".to_string(),
            })
    }

    async fn update_last_login(&self, user_id: Uuid) -> Result<()> {
        sqlx::query("UPDATE users SET last_login = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(self.db_pool)
            .await?;
        Ok(())
    }
}