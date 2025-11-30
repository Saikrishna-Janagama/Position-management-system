# Position Management System 📊

Advanced leverage trading platform built on Solana with real-time dashboard.



## 🎯 Features

✅ **Smart Contract (Solana)**
- User account initialization
- Position opening/closing
- Liquidation detection
- PnL tracking
- Margin calculations

✅ **Backend API (Rust)**
- 10+ REST endpoints
- Real-time metrics
- User & position management
- PnL analytics

✅ **Professional Dashboard**
- Real-time statistics
- Interactive forms
- Beautiful UI/UX
- System analytics
- Data export

## 🚀 Quick Start

### Build Smart Contract
```bash
anchor build
anchor test
anchor deploy --provider.cluster devnet
```

### Run Backend
```bash
cd backend
cargo run
```

Visit: **http://127.0.0.1:8080**

## 📚 API Endpoints

| Method | Endpoint | Purpose |
|--------|----------|---------|
| GET | `/health` | Health check |
| POST | `/user/initialize` | Create user account |
| GET | `/user/{address}` | Get user details |
| GET | `/user/{address}/pnl` | Get user PnL |
| POST | `/position/open` | Open new position |
| GET | `/position/{id}` | Get position details |
| POST | `/position/close` | Close position |
| GET | `/positions` | List all positions |
| GET | `/users` | List all users |
| GET | `/metrics` | System metrics |

## 🏗️ Architecture

```
┌─────────────────────┐
│  Web Dashboard      │
│  (HTML + JS)        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  REST API           │
│  (Actix-web/Rust)   │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  In-Memory DB       │
│  (HashMap)          │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Smart Contract     │
│  (Anchor/Solana)    │
└─────────────────────┘
```

## 🛠️ Tech Stack

- **Smart Contract:** Anchor Framework (Solana)
- **Backend:** Rust + Actix-web
- **Frontend:** HTML5 + JavaScript
- **Database:** In-memory (HashMap)
- **Deployment:** Render / Railway / Docker

## 📊 Example Usage

### Initialize User
```bash
curl -X POST http://127.0.0.1:8080/user/initialize \
  -H "Content-Type: application/json" \
  -d '{"address": "user123"}'
```

### Open Position
```bash
curl -X POST http://127.0.0.1:8080/position/open \
  -H "Content-Type: application/json" \
  -d '{
    "owner": "user123",
    "symbol": "BTC-PERP",
    "side": 1,
    "size": 1000000,
    "entry_price": 50000000000,
    "leverage": 10
  }'
```

### Get Metrics
```bash
curl http://127.0.0.1:8080/metrics
```

## 🎓 Learning Resources

- [Anchor Book](https://book.anchor-lang.com)
- [Solana Documentation](https://docs.solana.com)
- [Actix-web Guide](https://actix.rs)

## 🤝 Contributing

Feel free to fork and submit PRs!

## 📄 License

MIT License - Feel free to use this project

## 👤 Author

**Saikrishna Janagama**
- GitHub: https://github.com/Saikrishna-Janagama
- Email: Saikrishnajanagama68@gmail.com
- Ph num : +91 6281006609

---
