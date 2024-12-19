use super::orderbook::{Order, OrderBook};
use std::{collections::HashMap, fmt::format};
//BTCUSD
//BTC->base
//USDC->quote

//in short the amount we gonna get in terms of quote while selling Base
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}
impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    pub fn to_string(self) -> String {
        format!("{} to {}", self.base, self.quote)
    }
}
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}
impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }
    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("opening new orderbook for {:?}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: f64,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_order(price, order);
                println!("Placed limit order at price lever {:}", price);
                Ok(())
            }
            None => Err(format!(
                "The orderbook for trading pair ({}) doesnt exist",
                pair.to_string()
            )),
        }
    }
}
