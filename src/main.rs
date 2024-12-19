mod Engine;
use Engine::engine::{MatchingEngine, TradingPair};
use Engine::orderbook::{BidorAsk, Order, OrderBook};
fn main() {
    let buy_order_from_man = Order::new(BidorAsk::Bid, 5.5);
    let buy_order_from_human = Order::new(BidorAsk::Bid, 7.0);
    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, buy_order_from_man);
    orderbook.add_order(4.4, buy_order_from_human);

    //println!("{:?}", orderbook);

    let mut MatchingEngine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USDC".to_string());
    MatchingEngine.add_new_market(pair.clone());

    let buy_order = Order::new(BidorAsk::Bid, 8.5);
    let ETH_pair = TradingPair::new("ETH".to_string(), "USDT".to_string());
    MatchingEngine
        .place_limit_order(pair, 10.00, buy_order)
        .unwrap();
}
