use actix_web::{Responder, HttpResponse, get, web,post};
use std::collections::VecDeque;
use std::sync::{Arc};//, RwLock};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::order_book_mod::{OrderBook,Order,Match};

//pub type SharedMatchList = Arc<RwLock<VecDeque<Match>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientOrder{
    id:u8,
    o_type:u8,
    side:bool,
    price:u32,
    size:u32

}

pub struct RequestData {
    pub input: String,
}

pub struct ResponseData {
    pub output: String,
}

pub fn handle_request(data: RequestData) -> ResponseData {
    // Your processing logic here
    ResponseData {
        output: format!("Processed: {}\n", data.input),
    }
}

pub async fn index(req_body: String) -> impl Responder {
    // Convert request body into your input type
    let input = RequestData { input: req_body };

    // Process it
    let result = handle_request(input);

    // Convert response
    HttpResponse::Ok().body(result.output)
}
/*
#[get("/data")]
pub async fn send_market_data(order_book: web::Data<Arc<RwLock<OrderBook>>>) -> impl Responder {
    let book = order_book.read().await;
    HttpResponse::Ok().json(&*book)
}


#[get("/data2")]
pub async fn send_market_data2(order_book: web::Data<OrderBook>) -> impl Responder {
    let book = &order_book;
    HttpResponse::Ok().json(&*book)
}


#[get("/data2")]
pub async fn send_market_data2(order_book: web::Data<OrderBook>) -> impl Responder {

    HttpResponse::Ok().json(order_book.get_ref())
}
    */

#[get("/data")]
pub async fn send_market_data(
    order_book: web::Data<Arc<RwLock<OrderBook>>>
) -> impl Responder {
    let book = order_book.read().await; // or .await if using tokio::RwLock
    HttpResponse::Ok().json(&*book)
}



/*
#[post("/submit")]
pub async fn submit_order(
    order: web::Json<ClientOrder>,
    order_book: web::Data<Arc<RwLock<OrderBook>>>,
) -> impl Responder {
    let mut book = order_book.write().await;
    // Example: push to bid or ask depending on side
    if order.side {
        book.bid[order.price as usize].push(order.into_inner());
    } else {
        book.ask[order.price as usize].push(order.into_inner());
    }

    HttpResponse::Ok().json("Order received and added to book")
}




#[post("/submit")]
pub async fn submit_order(order: web::Json<ClientOrder>) -> impl Responder {
    //println!("{:?}",order.side);
    
    HttpResponse::Ok().json(format!("Order: {:?} received and added to book",order))
}
*/



#[post("/submit")]
pub async fn submit_order(
    c_order: web::Json<ClientOrder>,
    order_book: web::Data<Arc<RwLock<OrderBook>>>,
    matches_l: web::Data<Arc<RwLock<VecDeque<Match>>>>
    ) -> impl Responder {

    let mut book = order_book.write().await;
    let mut list_m= matches_l;
    // transform Client order in order
    // Queue the new order
    let fin_order:Order=Order { id: (c_order.id), modify: (0), partial: (0), size: (c_order.size), price: (c_order.price), side: (c_order.side), o_type: (c_order.o_type) };
    
    let mut incoming_orders:VecDeque<Order> = std::collections::VecDeque::new();
    incoming_orders.push_back(fin_order);

    // Prepare a place to store matches
    //let mut list_matches = Vec::new();
    // Process the order
    book.incoming_orders_processor(&mut incoming_orders, &mut list_m).await;
    // Optional: return matches or a confirmation
    HttpResponse::Ok().body("Order processed")
    
}