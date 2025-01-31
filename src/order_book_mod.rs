use std::collections::VecDeque;
use std::usize;


pub const ORDER_BOOK_LENGTH: usize = 10;

#[derive(Clone, Debug)]
/// - Side: Bid=true - Ask=false
pub struct Order {
    pub id: u8,
    pub size: u32,
    pub price: u64,
    pub side: bool,
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
/// - 4 -> Cross Ask: a new Ask order crosses the spread (Bid is the older resting order)
/// 
/// The 3 and 4 are useless if the trades are made only during the new order handling
#[derive(Debug)]
pub struct BestAB{
    pub ask_p:usize,
    pub bid_p:usize,
    pub ask_s:u32,
    pub bid_s:u32,
    pub state:u8
}

impl BestAB {

    pub fn new() -> BestAB {

    BestAB {
        ask_p:0,
        bid_p:0,
        ask_s:0,
        bid_s:0,
        state:0
        }
    
    }
}


pub struct ResultMatch{
    top_book:BestAB,
    o_bid: VecDeque<Order>,
    o_ask: VecDeque<Order>,
    is_match: bool
}


/// # Order Book
/// The most important part of the exchange
/// ## Functions
/// - ### new() -> OrderBook 
/// 
/// - ### inserter(&mut self,order: Order)
/// 
/// - ### rem(&mut self,side:bool,price: usize)->Order
/// 
/// - ### top_book(&self)->BestAB
#[derive(Debug)]
pub struct OrderBook {
    pub ask: [Box<VecDeque<Order>>; ORDER_BOOK_LENGTH],
    pub bid: [Box<VecDeque<Order>>; ORDER_BOOK_LENGTH],
    pub top_book : BestAB
}


impl OrderBook {

    pub fn new() -> OrderBook {
        // Create fixed-length arrays of `Box<VecDeque<Order>>`
        let ask = array_init::array_init(|_| Box::new(VecDeque::new()));
        let bid = array_init::array_init(|_| Box::new(VecDeque::new()));
        let top_book= BestAB::new();
        OrderBook { ask, bid ,top_book }
    }

    pub fn inserter(&mut self,order: Order){
    
        if order.side==true {
            self.bid[order.price as usize].push_back(order);
        } else if order.side==false {
            self.ask[order.price as usize].push_back(order);    
        }
        
    }

    /// _size: remove
    pub fn rem(&mut self,side:bool,price: usize)->Order{
       
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

    pub fn matching(&mut self)->ResultMatch{
        let top= self.top_book();
        if top.state==1{
            ResultMatch{
                top_book:top,
                o_bid: VecDeque::new(),
                o_ask: VecDeque::new(),
                is_match:false
            }
        }else{
            let mut ask_v:VecDeque<Order>=VecDeque::new();
            let mut bid_v:VecDeque<Order>=VecDeque::new();

            let bid_vol:u32=0;
            let bid_vol:u32=0;

            if top.state==1{
                bid_v.push_back(self.rem(true, top.bid_p)); 
                ask_v.push_back(self.rem(false, top.ask_p)); 
            };

            ResultMatch{
                top_book:top,
                o_bid: bid_v,
                o_ask: ask_v,
                is_match:true
            }
        }
    }

}




