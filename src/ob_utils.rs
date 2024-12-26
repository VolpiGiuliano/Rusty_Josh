use std::borrow::BorrowMut;

use crate::Order;
use crate::OrderBook;

pub struct BestBA{
    ask_size: u32,
    ask_price: f64,
    bid_size: u32,
    bid_price: f64
}


pub fn rem(side:bool,_size:i32,price: usize,or_bo: &mut OrderBook)->Order{
    or_bo.borrow_mut();

    if side==true {
        or_bo.bid[price].pop_front().unwrap()
    } else {
        or_bo.ask[price].pop_front().unwrap()
    }
    
}

pub fn inserter(order: Order,or_bo: &mut OrderBook){

    or_bo.borrow_mut();

    if order.side==true {
        or_bo.bid[order.price as usize].push_back(order);
    } else if order.side==false {
        or_bo.ask[order.price as usize].push_back(order);    
    }
    
}


pub fn best_bid_ask(or_bo: &mut OrderBook)->BestBA{
    //or_bo.borrow_mut();
    or_bo;

    
}
