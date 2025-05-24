use actix_web::{Responder, HttpResponse, get, web,post};
use std::collections::VecDeque;
use std::sync::{Arc};//, RwLock};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::order_book_mod::{OrderBook,Order,Match};


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


#[get("/data")]
pub async fn send_market_data(
    order_book: web::Data<Arc<RwLock<OrderBook>>>
) -> impl Responder {
    let book = order_book.read().await; // or .await if using tokio::RwLock
    HttpResponse::Ok().json(&*book)
}



#[post("/submit")]
pub async fn submit_order(
    c_order: web::Json<ClientOrder>,
    order_book: web::Data<Arc<RwLock<OrderBook>>>,
    matches_l: web::Data<Arc<RwLock<VecDeque<Match>>>>
    ) -> impl Responder {

    let mut book = order_book.write().await;
    let mut list_m= matches_l;
    let fin_order:Order = Order {
        id: (c_order.id),
        modify: (0),
        partial: (0),
        size: (c_order.size),
        price: (c_order.price),
        side: (c_order.side),
        o_type: (c_order.o_type)
        };
    
    let mut incoming_orders:VecDeque<Order> = std::collections::VecDeque::new();
    incoming_orders.push_back(fin_order);
    book.incoming_orders_processor(&mut incoming_orders, &mut list_m).await;
    HttpResponse::Ok().body("Order processed")
    
}