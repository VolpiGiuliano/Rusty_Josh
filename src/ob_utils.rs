use std::borrow::BorrowMut;

use crate::Order;
use crate::OrderBook;


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