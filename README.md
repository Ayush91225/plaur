# Plaur - AI-First Commerce Intelligence Platform

Plaur is an enterprise-grade, AI-first Commerce & Marketing Intelligence Operating System built in Rust for Shopify merchants.

## Architecture

### Microservices
- **Auth Service**: User authentication and authorization
- **API Gateway**: Central API routing and rate limiting
- **Ingestion Service**: Shopify data ingestion and webhooks
- **Analytics Service**: Business intelligence and reporting
- **AI Service**: Conversational AI and recommendations
- **Billing Service**: Shopify App Billing integration
- **Notification Service**: Alerts and communications

### Core Crates
- **plaur-common**: Shared utilities, models, and configurations
- **plaur-auth**: Authentication and authorization logic
- **plaur-api**: API gateway implementation
- **plaur-ingestion**: Data ingestion and processing
- **plaur-analytics**: Analytics and reporting engine
- **plaur-ai**: AI/ML services and RAG implementation
- **plaur-billing**: Shopify billing integration
- **plaur-notifications**: Notification system

### Lambda Functions
- **shopify-webhook**: Process Shopify webhooks
- **trend-processor**: Process market trend data
- **report-generator**: Generate automated reports
- **anomaly-detector**: Detect business anomalies

## Features

### Core Platform
- Multi-tenant SaaS architecture
- Role-based access control
- Audit logging and compliance
- Real-time analytics and dashboards
- Anomaly detection and alerting

### Shopify Integration
- Full Shopify Admin API integration
- Real-time webhook processing
- Product, order, and customer sync
- Inventory and pricing management
- Native Shopify App Billing

### AI & Intelligence
- Conversational AI with RAG (Quadrant DB)
- Voice summaries (Sarvam API)
- Trend analysis and mapping
- Automated recommendations
- Predictive analytics

### Analytics & Reporting
- Real-time business metrics
- Cohort and RFM analysis
- Marketing attribution
- Weekly automated reports
- Custom dashboard widgets

## Technology Stack

- **Language**: Rust 1.75+
- **Web Framework**: Axum
- **Database**: PostgreSQL with SQLx
- **Cache**: Redis
- **Vector DB**: Quadrant DB
- **Message Queue**: AWS SQS/SNS
- **Storage**: AWS S3
- **Deployment**: AWS Lambda + Docker
- **Monitoring**: Prometheus + Grafana

## Development Setup

### Prerequisites
- Rust 1.75+
- Docker and Docker Compose
- PostgreSQL 15+
- Redis 7+

### Local Development

1. Clone the repository:
```bash
git clone <repository-url>
cd plaur
```

2. Start infrastructure services:
```bash
docker-compose up -d postgres redis qdrant
```

3. Set up environment variables:
```bash
cp config/default.yaml config/local.yaml
# Edit config/local.yaml with your settings
```

4. Run database migrations:
```bash
cargo install sqlx-cli
sqlx migrate run
```

5. Start services:
```bash
# Auth service
PLAUR_SERVICE_TYPE=auth-service cargo run

# API Gateway
PLAUR_SERVICE_TYPE=api-gateway cargo run

# Ingestion service
PLAUR_SERVICE_TYPE=ingestion-service cargo run
```

### Building for Production

Build all services for AWS Lambda:
```bash
./build.sh
```

Build Docker image for Lambda:
```bash
docker build --platform linux/x86_64 -t plaur-auth:latest -f Dockerfile.lambda .
docker create --name temp-plaur plaur-auth:latest
docker cp temp-plaur:/var/runtime/bootstrap ./bootstrap
docker rm temp-plaur
zip -j plaur-deployment.zip bootstrap
```

## Configuration

Configuration is managed through YAML files in the `config/` directory:
- `default.yaml`: Base configuration
- `development.yaml`: Development overrides
- `production.yaml`: Production overrides

Environment variables can override any configuration using the `PLAUR__` prefix:
```bash
PLAUR__DATABASE__URL=postgresql://user:pass@host:5432/db
PLAUR__REDIS__URL=redis://host:6379
```

## API Documentation

### Authentication
```bash
# Register user
POST /api/v1/auth/register
{
  "email": "user@example.com",
  "password": "password123",
  "first_name": "John",
  "last_name": "Doe",
  "role": "owner"
}

# Login
POST /api/v1/auth/login
{
  "email": "user@example.com",
  "password": "password123"
}
```

### Store Management
```bash
# Connect Shopify store
POST /api/v1/stores/connect
Authorization: Bearer <token>
{
  "shop_domain": "mystore.myshopify.com",
  "access_token": "shopify_access_token"
}

# Get store analytics
GET /api/v1/stores/{store_id}/analytics
Authorization: Bearer <token>
```

### AI Chat
```bash
# Start chat session
POST /api/v1/ai/chat
Authorization: Bearer <token>
{
  "message": "What were my sales this week?",
  "store_id": "uuid"
}
```

## Deployment

### AWS Lambda Deployment

1. Build the application:
```bash
./build.sh
```

2. Deploy using AWS CLI or CDK:
```bash
aws lambda create-function \
  --function-name plaur-auth \
  --runtime provided.al2 \
  --role arn:aws:iam::account:role/lambda-role \
  --handler bootstrap \
  --zip-file fileb://plaur-deployment.zip
```

### Environment Variables for Production

Required environment variables:
- `PLAUR__DATABASE__URL`: PostgreSQL connection string
- `PLAUR__REDIS__URL`: Redis connection string
- `PLAUR__SHOPIFY__APP_SECRET`: Shopify app secret
- `PLAUR__AI__OPENAI_API_KEY`: OpenAI API key
- `PLAUR__AI__QUADRANT_URL`: Quadrant DB URL
- `PLAUR__SECURITY__JWT_SECRET`: JWT signing secret

## Monitoring and Observability

### Metrics
- Request latency and throughput
- Database connection pool metrics
- Cache hit/miss rates
- AI model inference times
- Shopify API rate limits

### Logging
- Structured JSON logging with tracing
- Request/response logging
- Error tracking with Sentry integration
- Audit logs for sensitive operations

### Health Checks
Each service exposes health check endpoints:
- `GET /health`: Basic health status
- `GET /health/ready`: Readiness probe
- `GET /health/live`: Liveness probe

## Security

### Authentication & Authorization
- JWT-based authentication
- Role-based access control (RBAC)
- API key management for integrations
- Session management with Redis

### Data Protection
- Encryption at rest and in transit
- PII data anonymization
- GDPR compliance features
- SOC2 Type II controls

### Shopify Integration Security
- OAuth 2.0 flow implementation
- Webhook signature verification
- Least-privilege API scopes
- Secure token storage

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run the test suite: `cargo test`
6. Submit a pull request

## License

Copyright (c) 2024 Plaur Inc. All rights reserved.

This is proprietary software. Unauthorized copying, modification, distribution, or use is strictly prohibited.