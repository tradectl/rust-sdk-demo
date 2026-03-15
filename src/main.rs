mod strategy;

fn main() {
    tradectl_sdk::paper::run("demo-config.json", |strat| {
        Box::new(strategy::DemoStrategy::from_config(strat))
    });
}
