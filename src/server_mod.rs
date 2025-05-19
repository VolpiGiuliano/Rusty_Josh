use actix_web::{Responder, HttpResponse, get, web,post};
use crate::order_book_mod::{OrderBook};
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ClientOrder{
    o_type:u8,

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
    let book = order_book.read().unwrap(); // or .await if using tokio::RwLock
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
*/



#[post("/submit")]
pub async fn submit_order(order: web::Json<ClientOrder>) -> impl Responder {

    HttpResponse::Ok().json(format!("Order: {:?} received and added to book",order))
}