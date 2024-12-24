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

#[derive(Debug)]
pub struct BestAB{
    pub ask:usize,
    pub bid:usize

}

pub static ORDER_BOOK_LENGHT: usize=10;



