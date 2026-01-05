-- Create product status enum
CREATE TYPE product_status AS ENUM ('active', 'draft', 'archived');

-- Create financial status enum
CREATE TYPE financial_status AS ENUM ('pending', 'authorized', 'partially_paid', 'paid', 'partially_refunded', 'refunded', 'voided');

-- Create fulfillment status enum
CREATE TYPE fulfillment_status AS ENUM ('fulfilled', 'null', 'partial', 'restocked');

-- Create products table
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES stores(id) ON DELETE CASCADE,
    shopify_product_id BIGINT NOT NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    vendor VARCHAR(255),
    product_type VARCHAR(255),
    handle VARCHAR(255) NOT NULL,
    status product_status NOT NULL DEFAULT 'active',
    tags TEXT[] DEFAULT '{}',
    images JSONB DEFAULT '[]',
    variants JSONB DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(store_id, shopify_product_id)
);

-- Create customers table
CREATE TABLE customers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES stores(id) ON DELETE CASCADE,
    shopify_customer_id BIGINT,
    email VARCHAR(255),
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(50),
    total_spent DECIMAL(10,2) DEFAULT 0,
    orders_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(store_id, shopify_customer_id)
);

-- Create orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES stores(id) ON DELETE CASCADE,
    shopify_order_id BIGINT NOT NULL,
    order_number VARCHAR(50) NOT NULL,
    customer_id UUID REFERENCES customers(id),
    email VARCHAR(255),
    total_price DECIMAL(10,2) NOT NULL,
    subtotal_price DECIMAL(10,2) NOT NULL,
    total_tax DECIMAL(10,2) DEFAULT 0,
    total_discounts DECIMAL(10,2) DEFAULT 0,
    currency VARCHAR(3) NOT NULL,
    financial_status financial_status NOT NULL,
    fulfillment_status fulfillment_status,
    source_name VARCHAR(100),
    referring_site VARCHAR(500),
    landing_site VARCHAR(500),
    line_items JSONB DEFAULT '[]',
    shipping_address JSONB,
    billing_address JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(store_id, shopify_order_id)
);

-- Create indexes
CREATE INDEX idx_products_store_id ON products(store_id);
CREATE INDEX idx_products_shopify_id ON products(shopify_product_id);
CREATE INDEX idx_products_status ON products(status);
CREATE INDEX idx_products_handle ON products(handle);

CREATE INDEX idx_customers_store_id ON customers(store_id);
CREATE INDEX idx_customers_email ON customers(email);
CREATE INDEX idx_customers_shopify_id ON customers(shopify_customer_id);

CREATE INDEX idx_orders_store_id ON orders(store_id);
CREATE INDEX idx_orders_customer_id ON orders(customer_id);
CREATE INDEX idx_orders_shopify_id ON orders(shopify_order_id);
CREATE INDEX idx_orders_financial_status ON orders(financial_status);
CREATE INDEX idx_orders_created_at ON orders(created_at);