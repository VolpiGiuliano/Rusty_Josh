use std::{borrow::BorrowMut, usize};
use crate::stru::{self, BestAB};



pub fn top_book(size_ob: usize,or_bo: &mut stru::OrderBook)->stru::BestAB{

    or_bo.borrow_mut();

    let (mut b_ask, mut b_bid): (usize, usize) = (0, 0);
    let mut found_bid = false;
    let mut found_ask = false;

    let mut bid_index= size_ob-1;

    for ask_index  in 0..bid_index{

        println!("Ask index: {}",ask_index);
        println!("{:?}",or_bo.ask[ask_index]);
        
        if or_bo.ask[ask_index].is_empty(){
            println!("Empty Ask");
        }else {
            b_ask=ask_index as usize;
            found_ask=true
        };

        if found_bid ==false{

            if or_bo.bid[bid_index].is_empty(){
                println!("Empty Bid");
                bid_index = bid_index-1; 
            }else {
                b_bid=bid_index as usize;
                found_bid=true
            }

        };
        
        if found_ask & found_bid{
            break;
        }

    };

    
    let best_ba:BestAB=BestAB{
        ask: b_ask,
        bid : b_bid
    };

    return best_ba;
}


pub fn rem(side:bool,_size:i32,price: usize,or_bo: &mut stru::OrderBook)->stru::Order{
    or_bo.borrow_mut();

    if side==true {
        or_bo.bid[price].pop_front().unwrap()
    } else {
        or_bo.ask[price].pop_front().unwrap()
    }
    
}

pub fn inserter(order: stru::Order,or_bo: &mut stru::OrderBook){

    or_bo.borrow_mut();

    if order.side==true {
        or_bo.bid[order.price as usize].push_back(order);
    } else if order.side==false {
        or_bo.ask[order.price as usize].push_back(order);    
    }
    
}

/// - Side: true=bid false=ask
pub fn volume_calculator(side: bool,price:usize,or_bo: &mut stru::OrderBook)-> u32{
    or_bo.borrow_mut();

    let mut size: u32=0;
    if side{
    println!("-------------------------"); 
    for ord in or_bo.bid[price].iter(){
        size+= ord.size;
        println!("{:?}",*ord);
    }
    println!("-------------------------");
    } else { 
        for ord in or_bo.ask[price].iter(){
            size+= ord.size;
            println!("{:?}",*ord);
        }
        println!("-------------------------");
    } 
    return size;
}