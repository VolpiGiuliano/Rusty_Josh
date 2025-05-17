use std::collections::{VecDeque};
use std::usize;


pub const ORDER_BOOK_LENGTH: usize = 10;

#[derive(Clone,Copy, Debug)]
/// - Side: Bid=true - Ask=false
/// - modify: Starts at 0 and at every modification is added by 1
/// - partial: 0 if no partial match was done, increment by 1 at every partial match
pub struct Order {
    pub id: u8,
    pub modify: u8,
    pub partial: u8,
    pub size: u32,
    pub price: u32,
    pub side: bool,
    pub o_type:u8
}


#[derive(Debug)]
/// Struct that 
/// - resting:
///     - true = bid was the resting order
///     - false = ask was the resting order
/// Add the partial information or figure out how to point towards that information
pub struct Match{
    id_b:u8,
    id_a:u8,
    volume_filled:u32,
    price:u32,
//    time_stamp:,
    resting: bool
}

pub struct ResultMatch{
    top_book:BestAB,
    o_bid: VecDeque<Order>,
    o_ask: VecDeque<Order>,
    is_match: bool
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
///  
/// *It can be useless, consider to eliminate it*
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



/// # Order Book
/// The most important part of the exchange, it has the order book it self and the top_book struct usefull
/// to handle informations and trades without the need to iterate the entire book more more than necessary.
/// ## Funtions
/// - new() -> OrderBook: Usefull to initialize it
/// 
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


    pub fn rem(&mut self,side:bool,price: usize)->Order{
       
        if side==true {
            self.bid[price].pop_front().unwrap()
        } else {
            self.ask[price].pop_front().unwrap()
        }
        
    }


    pub fn top_book_refresh(&mut self){

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
        
        self.top_book= best_ba;
    }


    /// - Side: true=bid false=ask
    pub fn volume_calculator(&self,side: bool,price:usize)-> u32{

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

    /// # Input and output Engine processor
    /// The Matching Engine has as an input a list of new orders and it outputs a list of Matches.
    /// To handle this two lists and to not overcomplicate the Engine we use this function
    pub fn incoming_orders_processor(&mut self,list_order:&mut VecDeque<Order>, list_match:&mut VecDeque<Match> ){
        while let Some(order_in) = list_order.pop_front(){
            // list_match.append(&mut self.new_order_handling(order_in)); // good for now
            list_match.append(&mut self.new_order_handling(order_in)); // test
        }
    }


    /// # Matching Engine for Limit Orders
    /// The function handles new Limit Orders
    /// 
    /// 
    pub fn new_order_handling(&mut self,mut new_order:Order)->VecDeque<Match>{
        
        let mut matches_vec: VecDeque<Match>= Default::default();

        // Loop usefull to manage a partial fill
        // The loop should be more specific to the side (you don't need
        // to check again if it is a bid or ask)
        loop {
            
            // New Bid, examine the Ask
            // there is a hit
            if new_order.side && new_order.price >= self.top_book.ask_p as u32{
                println!("New bid");
            
                // Modify rest
                if new_order.size < self.ask[self.top_book.ask_p].front().unwrap().size {
                    
                    // Safe
                    if let Some(rest_order) = self.ask[self.top_book.ask_p].front_mut(){
                        rest_order.size -= new_order.size;
                        rest_order.partial +=1;

                        matches_vec.push_back(Match{
                            id_b:new_order.id,
                            id_a:self.ask[self.top_book.ask_p].front().unwrap().id,
                            volume_filled:new_order.size ,
                            price:self.ask[self.top_book.ask_p].front().unwrap().price,
                        //    time_stamp:,
                            resting: false
                        } );
                        self.top_book_refresh();
                        return matches_vec

                    }
                    

                //------------------------------------------------------------------------------
                // Modify new
                }else if  new_order.size > self.ask[self.top_book.ask_p].front().unwrap().size {
                    new_order.size -=self.ask[self.top_book.ask_p].front().unwrap().size;
                    
                    matches_vec.push_back(Match{
                        id_b:new_order.id,
                        id_a:self.ask[self.top_book.ask_p].front().unwrap().id,
                        volume_filled:self.ask[self.top_book.ask_p].front().unwrap().size,
                        price:self.ask[self.top_book.ask_p].front().unwrap().price,
                    //    time_stamp:,
                        resting: false
                    });

                    self.ask[self.top_book.ask_p].pop_front();
                    self.top_book_refresh();
                    // The new order needs to be looped back and processed
                    continue;
                    
                }else if  new_order.size == self.ask[self.top_book.ask_p].front().unwrap().size {

                    
                    matches_vec.push_back(Match{
                        id_b:new_order.id,
                        id_a:self.ask[self.top_book.ask_p].front().unwrap().id,
                        volume_filled:new_order.size,
                        price:self.ask[self.top_book.ask_p].front().unwrap().price,
                    //    time_stamp:,
                        resting: false
                    });

                    self.ask[self.top_book.ask_p].pop_front();
                    self.top_book_refresh();
                    return matches_vec;

                }


////////////////////////////////////ASK///////////////////////////////////////////////////    
            
            }else if new_order.side==false && new_order.price <= self.top_book.bid_p as u32 {// New Bid, examine the Ask
                println!("New ask");



                if new_order.size < self.bid[self.top_book.bid_p].front().unwrap().size {
                    
                    // Safe
                    if let Some(rest_order) = self.bid[self.top_book.bid_p].front_mut(){
                        rest_order.size -= new_order.size;
                        rest_order.partial +=1;

                        matches_vec.push_back(Match{
                            id_b: self.bid[self.top_book.bid_p].front().unwrap().id,
                            id_a: new_order.id,
                            volume_filled: new_order.size ,
                            price:self.bid[self.top_book.bid_p].front().unwrap().price,
                        //    time_stamp:,
                            resting: false
                        } );
                        self.top_book_refresh();
                        return matches_vec

                
                    }

            
                }else if  new_order.size > self.bid[self.top_book.bid_p].front().unwrap().size {
                    new_order.size -=self.bid[self.top_book.bid_p].front().unwrap().size;

                    matches_vec.push_back(Match{
                        id_b: self.bid[self.top_book.bid_p].front().unwrap().id,
                        id_a: new_order.id,
                        volume_filled:self.bid[self.top_book.bid_p].front().unwrap().size,
                        price:self.bid[self.top_book.bid_p].front().unwrap().price,
                    //    time_stamp:,
                        resting: false
                    });

                    self.bid[self.top_book.bid_p].pop_front();
                    self.top_book_refresh();
                    // The new order needs to be looped back and processed
                    continue;
                
                }else if  new_order.size == self.bid[self.top_book.bid_p].front().unwrap().size {

                    
                    matches_vec.push_back(Match{
                        id_b: self.bid[self.top_book.bid_p].front().unwrap().id,
                        id_a: new_order.id,
                        volume_filled:new_order.size,
                        price:self.bid[self.top_book.bid_p].front().unwrap().price,
                    //    time_stamp:,
                        resting: false
                    });

                    self.bid[self.top_book.bid_p].pop_front();
                    self.top_book_refresh();
                    return matches_vec;

                }
                


            } else {
                self.inserter(new_order);
                println!("Inserted no match");
                self.top_book_refresh();
                return matches_vec
            }

            self.top_book_refresh();
        
        } // loop

    
    }


    /// # Matching Engine for Market Orders
    /// The function handles new Market Orders.
    /// 
    /// 
    pub fn new_m_order_handling(&mut self,mut new_order:Order)->VecDeque<Match>{
        
        let mut matches_vec: VecDeque<Match>= Default::default();
        // Check if you have a Market Order
        if new_order.o_type != 0{
            println!("ERROR a NON market order entered in the handling");
            return matches_vec;
        }

        // Loop usefull to manage a partial fill
        loop {
            // new order is a bid
            if new_order.side == true{
                if new_order.size < self.top_book.ask_s{
                    // Safe
                    if let Some(rest_order) = self.ask[self.top_book.ask_p].front_mut(){
                        rest_order.size -= new_order.size;
                        rest_order.partial +=1;

                        matches_vec.push_back(Match{
                                id_b:new_order.id,
                                id_a:self.bid[self.top_book.bid_p].front().unwrap().id,
                                volume_filled:new_order.size ,
                                price:self.bid[self.top_book.bid_p].front().unwrap().price,
                            //    time_stamp:,
                                resting: false
                            } );

                        self.top_book_refresh();
                        return matches_vec
                    }
                }else if new_order.size > self.top_book.ask_s {
                    


                }


               
            }else { // new order is an ask  
                
            }
        }
        matches_vec
    }

    pub fn tot_order_handling(&mut self,mut new_order:Order)->VecDeque<Match>{
        
        let mut matches_vec: VecDeque<Match>= Default::default();

        // Loop usefull to manage a partial fill
        // The loop should be more specific to the side (you don't need
        // to check again if it is a bid or ask)
        loop {
            
            // New Bid, examine the Ask
            // there is a hit
            if new_order.side && ((new_order.price >= self.top_book.ask_p as u32 && new_order.o_type==1)|| new_order.o_type==0){
                println!("New bid");
            
                // Modify rest
                if new_order.size < self.ask[self.top_book.ask_p].front().unwrap().size {
                    
                    // Safe
                    if let Some(rest_order) = self.ask[self.top_book.ask_p].front_mut(){
                        rest_order.size -= new_order.size;
                        rest_order.partial +=1;

                        matches_vec.push_back(Match{
                            id_b:new_order.id,
                            id_a:self.ask[self.top_book.ask_p].front().unwrap().id,
                            volume_filled:new_order.size ,
                            price:self.ask[self.top_book.ask_p].front().unwrap().price,
                        //    time_stamp:,
                            resting: false
                        } );
                        self.top_book_refresh();
                        return matches_vec

                    }
                    

                //------------------------------------------------------------------------------
                // Modify new
                }else if  new_order.size > self.ask[self.top_book.ask_p].front().unwrap().size {
                    new_order.size -=self.ask[self.top_book.ask_p].front().unwrap().size;
                    
                    matches_vec.push_back(Match{
                        id_b:new_order.id,
                        id_a:self.ask[self.top_book.ask_p].front().unwrap().id,
                        volume_filled:self.ask[self.top_book.ask_p].front().unwrap().size,
                        price:self.ask[self.top_book.ask_p].front().unwrap().price,
                    //    time_stamp:,
                        resting: false
                    });

                    self.ask[self.top_book.ask_p].pop_front();
                    self.top_book_refresh();
                    // The new order needs to be looped back and processed
                    continue;
                    
                }else if  new_order.size == self.ask[self.top_book.ask_p].front().unwrap().size {

                    
                    matches_vec.push_back(Match{
                        id_b:new_order.id,
                        id_a:self.ask[self.top_book.ask_p].front().unwrap().id,
                        volume_filled:new_order.size,
                        price:self.ask[self.top_book.ask_p].front().unwrap().price,
                    //    time_stamp:,
                        resting: false
                    });

                    self.ask[self.top_book.ask_p].pop_front();
                    self.top_book_refresh();
                    return matches_vec;

                }


////////////////////////////////////ASK///////////////////////////////////////////////////    
            
            }else if new_order.side==false && ((new_order.price <= self.top_book.bid_p as u32 && new_order.o_type==1) || new_order.o_type==0) {// New Bid, examine the Ask
                println!("New ask");



                if new_order.size < self.bid[self.top_book.bid_p].front().unwrap().size {
                    
                    // Safe
                    if let Some(rest_order) = self.bid[self.top_book.bid_p].front_mut(){
                        rest_order.size -= new_order.size;
                        rest_order.partial +=1;

                        matches_vec.push_back(Match{
                            id_b: self.bid[self.top_book.bid_p].front().unwrap().id,
                            id_a: new_order.id,
                            volume_filled: new_order.size ,
                            price:self.bid[self.top_book.bid_p].front().unwrap().price,
                        //    time_stamp:,
                            resting: false
                        } );
                        self.top_book_refresh();
                        return matches_vec

                
                    }

            
                }else if  new_order.size > self.bid[self.top_book.bid_p].front().unwrap().size {
                    new_order.size -=self.bid[self.top_book.bid_p].front().unwrap().size;

                    matches_vec.push_back(Match{
                        id_b: self.bid[self.top_book.bid_p].front().unwrap().id,
                        id_a: new_order.id,
                        volume_filled:self.bid[self.top_book.bid_p].front().unwrap().size,
                        price:self.bid[self.top_book.bid_p].front().unwrap().price,
                    //    time_stamp:,
                        resting: false
                    });

                    self.bid[self.top_book.bid_p].pop_front();
                    self.top_book_refresh();
                    // The new order needs to be looped back and processed
                    continue;
                
                }else if  new_order.size == self.bid[self.top_book.bid_p].front().unwrap().size {

                    
                    matches_vec.push_back(Match{
                        id_b: self.bid[self.top_book.bid_p].front().unwrap().id,
                        id_a: new_order.id,
                        volume_filled:new_order.size,
                        price:self.bid[self.top_book.bid_p].front().unwrap().price,
                    //    time_stamp:,
                        resting: false
                    });

                    self.bid[self.top_book.bid_p].pop_front();
                    self.top_book_refresh();
                    return matches_vec;

                }
                


            } else {
                self.inserter(new_order);
                println!("Inserted no match");
                self.top_book_refresh();
                return matches_vec
            }

            self.top_book_refresh();
        
        } // loop

    
    }


}
