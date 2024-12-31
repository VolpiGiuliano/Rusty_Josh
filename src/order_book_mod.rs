use std::collections::VecDeque;
use std::usize;


pub const ORDER_BOOK_LENGTH: usize = 10;

#[derive(Clone, Debug)]
/// - Side: Bid=true - Ask=false
pub struct Order {
    pub id: u8,
    pub size: u32,
    pub price: f64,
    pub side: bool,
}

#[derive(Debug)]
pub struct OrderBook {
    pub ask: [Box<VecDeque<Order>>; ORDER_BOOK_LENGTH],
    pub bid: [Box<VecDeque<Order>>; ORDER_BOOK_LENGTH],
}



impl OrderBook {
    pub fn new() -> OrderBook {
        // Create fixed-length arrays of `Box<VecDeque<Order>>`
        let ask = array_init::array_init(|_| Box::new(VecDeque::new()));
        let bid = array_init::array_init(|_| Box::new(VecDeque::new()));

        OrderBook { ask, bid }
    }

    pub fn inserter(&mut self,order: Order){

        //self.borrow_mut();
    
        if order.side==true {
            self.bid[order.price as usize].push_back(order);
        } else if order.side==false {
            self.ask[order.price as usize].push_back(order);    
        }
        
    }


    pub fn rem(&mut self,side:bool,_size:i32,price: usize)->Order{
       
        if side==true {
            self.bid[price].pop_front().unwrap()
        } else {
            self.ask[price].pop_front().unwrap()
        }
        
    }




    pub fn top_book(&self)->BestAB{

        
    
        let (mut b_ask, mut b_bid): (usize, usize) = (0, 0);
        let mut state_ob:u8=0;
        let mut found_bid = false;
        let mut found_ask = false;
        let mut bid_index= ORDER_BOOK_LENGTH-1;
    
        for ask_index  in 0..bid_index{
    
            // if (not empty and not found)
            if !self.ask[ask_index].is_empty() & !found_ask{
                b_ask=ask_index as usize;
                found_ask=true
            }
    
    
            if found_bid ==false{
    
                if self.bid[bid_index].is_empty(){
                    
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
            ask_s: self.volume_calculator(false,b_ask),
            bid_s: self.volume_calculator(true,b_bid),
            state: state_ob
        };
    
        return best_ba;
    }


    /// - Side: true=bid false=ask
    pub fn volume_calculator(&self,side: bool,price:usize)-> u32{
        //or_bo.borrow_mut();

        let mut size: u32=0;

        if side{

            for ord in self.bid[price].iter(){
                size+= ord.size;
            }
        } else { 

            for ord in self.ask[price].iter(){
                size+= ord.size;
            }   
        } 
        return size;
    }


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




