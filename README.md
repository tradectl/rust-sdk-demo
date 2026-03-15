# tradectl-sdk demo

Paper trading demo using [tradectl/rust-sdk](https://github.com/tradectl/rust-sdk). Connects to a live exchange for real-time prices, runs a simple TP/SL strategy, and broadcasts state to the [monitor dashboard](https://github.com/tradectl/monitor).

## Quick start

```bash
# Terminal 1: start the monitor dashboard
cd ../monitor && npm install && npm run dev

# Terminal 2: run the demo
cargo run                              # uses demo-config.json
cargo run -- my-config.json            # custom config
```

Open [http://localhost:3002](http://localhost:3002) to see the live chart with entry, TP, and SL price lines.

## Configuration

Uses the same config format as production bots. Edit `demo-config.json`:

```json
{
    "api": {
        "provider": "Binance"
    },
    "monitor": {
        "host": "0.0.0.0",
        "port": 9100
    },
    "strats": [
        {
            "name": "demo01",
            "type": "Demo",
            "marketType": "LINEAR",
            "isEmulator": true,
            "pairs": ["BTCUSDT"],
            "orderSize": 0.001,
            "takeProfit": 0.15,
            "stopLoss": 0.10,
            "cooldownSecs": 10,
            "warmupTicks": 50
        }
    ]
}
```

| Field | Description |
|-------|-------------|
| `api.provider` | Exchange for price feed (`Binance`) |
| `monitor.port` | Monitor WebSocket server port |
| `strats[].name` | Strategy instance name |
| `strats[].type` | Strategy type |
| `strats[].marketType` | `LINEAR`, `INVERSE`, or `SPOT` |
| `strats[].isEmulator` | Paper trading mode (`true` = no real orders) |
| `strats[].pairs` | Trading pairs |
| `strats[].orderSize` | Order size per trade |
| `strats[].takeProfit` | Take profit percentage |
| `strats[].stopLoss` | Stop loss percentage |
| `strats[].cooldownSecs` | Seconds to wait between trades |
| `strats[].warmupTicks` | Ticks to wait before first trade |

## How it works

1. SDK's paper runner connects to the exchange's public WebSocket (read-only, no API keys)
2. Real-time prices feed into the strategy's `on_ticker()` method
3. Strategy returns `Action::MarketOpen` or `Action::ClosePosition`
4. Runner simulates fills locally — no real orders placed
5. State broadcasts to monitor dashboard via WebSocket

## Project structure

```
demo-config.json   # Bot config (same format as production)
src/
  main.rs          # Entry point (5 lines)
  strategy.rs      # Strategy implementation (Strategy trait)
```

## Requirements

- Rust 1.70+
- Internet connection (exchange WebSocket)
- No API keys needed (read-only public stream)
