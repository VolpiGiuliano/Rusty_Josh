use std::{borrow::BorrowMut, usize};
use crate::stru::{self, BestAB};



pub fn top_book(size_ob: usize,or_bo: &mut stru::OrderBook)->stru::BestAB{

    or_bo.borrow_mut();

    let (mut b_ask, mut b_bid): (usize, usize) = (0, 0);
    let mut state_ob:u8=0;
    let mut found_bid = false;
    let mut found_ask = false;
    let mut bid_index= size_ob-1;

    for ask_index  in 0..bid_index{

        // if (not empty and not found)
        if !or_bo.ask[ask_index].is_empty() & !found_ask{
            b_ask=ask_index as usize;
            found_ask=true
        }


        if found_bid ==false{

            if or_bo.bid[bid_index].is_empty(){
                
                bid_index -= 1; 
            }else {
                b_bid=bid_index as usize;
                found_bid=true
            }

        };
        
        if found_ask & found_bid{
            break;
        }

    };

    // Read doc of BestAB
    if b_ask==b_bid{
        state_ob=1
    }
    
    let best_ba:BestAB=BestAB{
        ask_p: b_ask,
        bid_p: b_bid,
        ask_s: volume_calculator(false,b_ask,or_bo),
        bid_s: volume_calculator(true,b_bid,or_bo),
        state: state_ob
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

        for ord in or_bo.bid[price].iter(){
            size+= ord.size;
        }
    } else { 

        for ord in or_bo.ask[price].iter(){
            size+= ord.size;
        }   
    } 
    return size;
}