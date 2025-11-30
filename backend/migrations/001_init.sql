-- Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    address VARCHAR(255) UNIQUE NOT NULL,
    total_collateral BIGINT DEFAULT 0,
    locked_collateral BIGINT DEFAULT 0,
    total_pnl BIGINT DEFAULT 0,
    position_count INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Positions Table
CREATE TABLE positions (
    id VARCHAR(255) PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    owner VARCHAR(255) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    side INT NOT NULL,  -- 1 for Long, 2 for Short
    size BIGINT NOT NULL,
    entry_price BIGINT NOT NULL,
    leverage INT NOT NULL,
    status INT NOT NULL,  -- 1: Open, 2: Closed, 3: Liquidated
    margin BIGINT NOT NULL,
    mark_price BIGINT DEFAULT 0,
    unrealized_pnl BIGINT DEFAULT 0,
    realized_pnl BIGINT DEFAULT 0,
    liquidation_price BIGINT DEFAULT 0,
    opened_at BIGINT NOT NULL,
    closed_at BIGINT DEFAULT 0,
    close_price BIGINT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Position History (Audit Trail)
CREATE TABLE position_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    position_id VARCHAR(255) NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    action VARCHAR(50) NOT NULL,  -- 'OPENED', 'MODIFIED', 'CLOSED', 'LIQUIDATED'
    old_values JSONB,
    new_values JSONB,
    reason VARCHAR(255),
    created_at TIMESTAMP DEFAULT NOW()
);

-- PnL Snapshots (Hourly/Daily Analytics)
CREATE TABLE pnl_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    position_id VARCHAR(255),
    snapshot_time TIMESTAMP NOT NULL,
    unrealized_pnl BIGINT,
    realized_pnl BIGINT,
    total_pnl BIGINT,
    margin_ratio DECIMAL(10, 4),
    created_at TIMESTAMP DEFAULT NOW()
);

-- Create Indexes for Performance
CREATE INDEX idx_positions_user_id ON positions(user_id);
CREATE INDEX idx_positions_symbol ON positions(symbol);
CREATE INDEX idx_positions_status ON positions(status);
CREATE INDEX idx_position_history_user_id ON position_history(user_id);
CREATE INDEX idx_position_history_position_id ON position_history(position_id);
CREATE INDEX idx_pnl_snapshots_user_id ON pnl_snapshots(user_id);
CREATE INDEX idx_pnl_snapshots_snapshot_time ON pnl_snapshots(snapshot_time);