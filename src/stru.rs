use std::collections::VecDeque;


#[derive(Clone)]
#[derive(Debug)]
/// - Side: Bid=true - Ask=false
pub struct Order{
    pub id:u8,
    pub size:u32,
    pub price:f64,
    pub side: bool
}


#[derive(Debug)]
pub struct OrderBook<'oblt>{
    pub ask: [&'oblt mut VecDeque<Order>;ORDER_BOOK_LENGHT],
    pub bid: [&'oblt mut VecDeque<Order>;ORDER_BOOK_LENGHT]
}

/// # Best Bid & Ask
/// 
/// Top view of the orderbook with the best bid and ask with
/// the volume and state of the spread.
/// 
/// ## state 
/// 
/// Flag usefull to know the state of the order book.
/// *IMPORTANT*: The in case of a cross spread you need to consider the price
/// of the older resting order for the transition
/// 
/// - 1 -> Equilibrium: we have a Bid price < Ask price
/// - 2 -> Contact: Bid=Ask
/// - 3 -> Cross Bid: a new Bid order crosses the spread (Ask is the older resting order)
/// - 3 -> Cross Ask: a new Ask order crosses the spread (Bid is the older resting order)
#[derive(Debug)]
pub struct BestAB{
    pub ask_p:usize,
    pub bid_p:usize,
    pub ask_s:u32,
    pub bid_s:u32,
    pub state:u8
}

pub static ORDER_BOOK_LENGHT: usize=10;



