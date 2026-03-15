use tradectl_sdk::strategy::*;
use tradectl_sdk::types::config::StratEntry;
use tradectl_sdk::types::{Side, TickerEvent};

pub struct DemoStrategy {
    tp_pct: f64,
    sl_pct: f64,
    cooldown_ms: u64,
    warmup_ticks: usize,
    tick_count: usize,
    entry_price: Option<f64>,
    tp_price: f64,
    sl_price: f64,
    cooldown_until: u64,
}

impl DemoStrategy {
    pub fn from_config(strat: &StratEntry) -> Self {
        Self {
            tp_pct: strat.get_f64_or("takeProfit", 0.15),
            sl_pct: strat.get_f64_or("stopLoss", 0.10),
            cooldown_ms: (strat.get_f64_or("cooldownSecs", 10.0) * 1000.0) as u64,
            warmup_ticks: strat.get_f64_or("warmupTicks", 50.0) as usize,
            tick_count: 0,
            entry_price: None,
            tp_price: 0.0,
            sl_price: 0.0,
            cooldown_until: 0,
        }
    }
}

impl Strategy for DemoStrategy {
    fn name(&self) -> &str {
        "demo"
    }

    fn describe(&self) -> &str {
        "Buy at market, TP/SL from config"
    }

    fn on_ticker(&mut self, ticker: &TickerEvent, ctx: &StrategyContext) -> Action {
        self.tick_count += 1;
        let mid = (ticker.bid_price + ticker.ask_price) * 0.5;

        if let Some(pos) = ctx.positions.first() {
            if mid >= self.tp_price {
                return Action::ClosePosition { position_id: pos.id, reason: CloseReason::TakeProfit };
            }
            if mid <= self.sl_price {
                return Action::ClosePosition { position_id: pos.id, reason: CloseReason::StopLoss };
            }
            return Action::Hold;
        }

        if ticker.timestamp_ms < self.cooldown_until {
            return Action::Hold;
        }

        if self.tick_count < self.warmup_ticks {
            return Action::Hold;
        }

        let entry = ticker.ask_price;
        self.entry_price = Some(entry);
        self.tp_price = entry * (1.0 + self.tp_pct / 100.0);
        self.sl_price = entry * (1.0 - self.sl_pct / 100.0);

        Action::MarketOpen {
            side: Side::Long,
            size: None,
        }
    }

    fn on_position_close(&mut self, _close: &CloseInfo, ctx: &StrategyContext) {
        self.entry_price = None;
        self.cooldown_until = ctx.timestamp_ms + self.cooldown_ms;
    }

    fn monitor_snapshot(&self, ctx: &StrategyContext, _ticker: &TickerEvent) -> MonitorSnapshot {
        let mut lines = Vec::new();

        if let Some(entry) = self.entry_price {
            lines.push(PriceLine {
                label: "Entry".into(),
                price: entry,
                color: "#3b82f6".into(),
                style: "solid".into(),
                line_width: 1,
                axis_label: true,
            });
            lines.push(PriceLine {
                label: "TP".into(),
                price: self.tp_price,
                color: "#22c55e".into(),
                style: "dashed".into(),
                line_width: 1,
                axis_label: true,
            });
            lines.push(PriceLine {
                label: "SL".into(),
                price: self.sl_price,
                color: "#ef4444".into(),
                style: "dashed".into(),
                line_width: 1,
                axis_label: true,
            });
        }

        let pos_data: Vec<serde_json::Value> = ctx.positions.iter().map(|p| {
            serde_json::json!({
                "side": format!("{:?}", p.side),
                "quantity": p.quantity,
                "entry_price": p.entry_price,
                "tp_price": self.tp_price,
                "sl_price": self.sl_price,
            })
        }).collect();

        MonitorSnapshot {
            price_lines: lines,
            state: serde_json::json!({
                "positions": pos_data,
                "tp": format!("{:.2}", self.tp_price),
                "sl": format!("{:.2}", self.sl_price),
            }),
        }
    }
}
