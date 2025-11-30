# Technical Documentation

## Leverage Tiers System

The system implements 5 leverage tiers based on position size:

| Tier | Max Leverage | Initial Margin | Maintenance Margin | Max Position |
|------|-------------|-----------------|-------------------|--------------|
| 1    | 20x         | 5.0%           | 2.5%              | Unlimited    |
| 2    | 50x         | 2.0%           | 1.0%              | 100,000      |
| 3    | 100x        | 1.0%           | 0.5%              | 50,000       |
| 4    | 500x        | 0.5%           | 0.25%             | 20,000       |
| 5    | 1000x       | 0.2%           | 0.1%              | 5,000        |

## Margin Calculations

### Initial Margin
```
Initial Margin = (Position Size × Entry Price) / Leverage
```

Example:
- Size: 1,000,000 tokens
- Entry Price: 50,000
- Leverage: 10x
- Initial Margin = (1,000,000 × 50,000) / 10 = 5,000,000 USDT

### Maintenance Margin
```
Maintenance Margin = Initial Margin × Maintenance Margin Ratio
```

### Margin Ratio
```
Margin Ratio = (Margin + Unrealized PnL) / Position Value
```

When Margin Ratio < Maintenance Margin Ratio → Position is liquidatable

### Liquidation Price

**For Long Positions:**
```
Liquidation Price = Entry Price × (1 - 1/Leverage + Maintenance Margin Ratio)
```

**For Short Positions:**
```
Liquidation Price = Entry Price × (1 + 1/Leverage - Maintenance Margin Ratio)
```

## PnL Calculation

### Unrealized PnL
```
For Long: PnL = Size × (Mark Price - Entry Price)
For Short: PnL = Size × (Entry Price - Mark Price)
```

### Realized PnL
```
Realized PnL = Unrealized PnL at close time
```

## Database Schema

See `backend/migrations/001_init.sql` for complete schema.

Key tables:
- `users` - User accounts
- `positions` - Current positions
- `position_history` - Audit trail
- `pnl_snapshots` - Historical analytics

## API Endpoints

### Position Management
- POST /position/open - Open position with leverage validation
- POST /position/close - Close position and calculate PnL
- GET /position/{id} - Get position details

### User Operations
- POST /user/initialize - Create user account
- GET /user/{address} - Get user details
- GET /user/{address}/pnl - Get user PnL summary

### Analytics
- GET /positions - List all positions
- GET /users - List all users
- GET /metrics - System metrics with leverage tiers

### Health
- GET /health - API health check

## Security & Validation

1. Leverage limits enforced by tier system
2. Position size validation against tier limits
3. Margin requirements checked before opening
4. Liquidation price calculated safely
5. Atomic state updates
6. Overflow protection with saturating arithmetic
