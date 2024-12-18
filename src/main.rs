use std::collections::HashMap;

#[derive(Debug)]
enum BidorAsk {
    Bid,
    Ask,
}
#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}
impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidorAsk::Bid => {
                let price = Price::new(price);
                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
            BidorAsk::Ask => {}
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}
impl Price {
    fn new(price: f64) -> Price {
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
struct Limit {
    price: Price,
    order: Vec<Order>,
}
impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            order: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.order.push(order);
    }
}
#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidorAsk,
}

impl Order {
    fn new(bid_or_ask: BidorAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }
}

fn main() {
    let buy_order_from_man = Order::new(BidorAsk::Bid, 5.5);
    let buy_order_from_human = Order::new(BidorAsk::Bid, 7.0);
    let mut orderbook = OrderBook::new();
    orderbook.add_order(4.4, buy_order_from_man);
    orderbook.add_order(4.4, buy_order_from_human);
    println!("{:?}", orderbook);
}
