-- Create user roles enum
CREATE TYPE user_role AS ENUM ('owner', 'admin', 'analyst', 'readonly', 'integrator');

-- Create subscription status enum
CREATE TYPE subscription_status AS ENUM ('trial', 'active', 'cancelled', 'suspended', 'past_due');

-- Create plan type enum
CREATE TYPE plan_type AS ENUM ('starter', 'growth', 'pro', 'enterprise');

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    role user_role NOT NULL DEFAULT 'readonly',
    is_active BOOLEAN NOT NULL DEFAULT true,
    email_verified BOOLEAN NOT NULL DEFAULT false,
    last_login TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create stores table
CREATE TABLE stores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    shopify_store_id BIGINT UNIQUE NOT NULL,
    shop_domain VARCHAR(255) UNIQUE NOT NULL,
    shop_name VARCHAR(255) NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    access_token TEXT NOT NULL,
    webhook_secret VARCHAR(255),
    subscription_status subscription_status NOT NULL DEFAULT 'trial',
    plan_type plan_type NOT NULL DEFAULT 'starter',
    trial_ends_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    timezone VARCHAR(50) DEFAULT 'UTC',
    currency VARCHAR(3) DEFAULT 'USD',
    country_code VARCHAR(2),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_stores_owner_id ON stores(owner_id);
CREATE INDEX idx_stores_shopify_id ON stores(shopify_store_id);
CREATE INDEX idx_stores_domain ON stores(shop_domain);