use std::collections::HashMap;
//This is orderbook
#[derive(Debug)]
pub enum BidorAsk {
    Bid,
    Ask,
}
#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}
impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);
        match order.bid_or_ask {
            BidorAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            BidorAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit: Limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}
impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional,
        }
    }
}
#[derive(Debug)]
pub struct Limit {
    price: Price,
    order: Vec<Order>,
}
impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            order: Vec::new(),
        }
    }
    pub fn total_volume(&self) -> f64 {
        let volume = self
            .order
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap();
        volume
    }
    pub fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.order.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }
            if market_order.is_filled() {
                break;
            }
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.order.push(order);
    }
}
#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidorAsk,
}

impl Order {
    pub fn new(bid_or_ask: BidorAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }
    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn total_volume() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_order = Order::new(BidorAsk::Bid, 100.0);
        let buy_order_1 = Order::new(BidorAsk::Bid, 50.0);
        limit.add_order(buy_order);
        limit.add_order(buy_order_1);

        assert_eq!(limit.total_volume(), 150.0);
    }
    #[test]
    fn limit_order_single_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order = Order::new(BidorAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(BidorAsk::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);
        println!("{:?}", limit);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.order.get(0).unwrap().size, 1.0);
    }
    #[test]
    fn limit_order_multi_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order_a = Order::new(BidorAsk::Bid, 100.0);

        let buy_limit_order_b = Order::new(BidorAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        let mut market_sell_order = Order::new(BidorAsk::Ask, 199.0);
        limit.fill_order(&mut market_sell_order);
        println!("{:?}", limit);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.order.get(1).unwrap().size, 1.0);
    }
}
