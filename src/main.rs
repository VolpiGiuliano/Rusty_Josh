use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

mod order_book_mod;
mod io_mod;
use order_book_mod::Order;

mod server_mod;
use server_mod::{index, send_market_data, submit_order};

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let mut order_book_i = order_book_mod::OrderBook::new();
    let mut order_book = Arc::new(RwLock::new(order_book_i));

    let mut incoming_orders: VecDeque<order_book_mod::Order>=VecDeque::new();
    let mut list_matches: VecDeque<order_book_mod::Match>=VecDeque::new();

    // Sample Orders
    let or_1: order_book_mod::Order= order_book_mod::Order { id: (1),modify: (0),partial:(0), size: (4), price:(5), side:(true), o_type: (1)};
    let or_2: order_book_mod::Order= order_book_mod::Order { id: (2),modify: (0),partial:(0), size: (3), price:(5), side:(true), o_type: (1)};
    let or_3: order_book_mod::Order= order_book_mod::Order { id: (3),modify: (0),partial:(0), size: (3), price:(7), side:(false), o_type: (1)};
    let or_4: order_book_mod::Order= order_book_mod::Order { id: (4),modify: (0),partial:(0), size: (9), price:(7), side:(false), o_type: (1)};
    let or_5: order_book_mod::Order= order_book_mod::Order { id: (5),modify: (0),partial:(0), size: (1), price:(8), side:(false), o_type: (1)};
    let or_6: order_book_mod::Order= order_book_mod::Order { id: (6),modify: (0),partial:(0), size: (1), price:(4), side:(true), o_type: (1)};
    let or_7: order_book_mod::Order= order_book_mod::Order { id: (7),modify: (0),partial:(0), size: (2), price:(3), side:(false), o_type: (1)};
    let or_8: order_book_mod::Order= order_book_mod::Order { id: (8),modify: (0),partial:(0), size: (2), price:(9), side:(true), o_type: (1)};
    let or_9: order_book_mod::Order= order_book_mod::Order { id: (9),modify: (0),partial:(0), size: (2), price:(9), side:(true), o_type: (1)};

    // test order market
    let or_10: order_book_mod::Order= order_book_mod::Order { id: (10),modify: (0),partial:(0), size: (2), price:(0), side:(true), o_type: (0)};
    let or_11: order_book_mod::Order= order_book_mod::Order { id: (11),modify: (0),partial:(0), size: (6), price:(0), side:(true), o_type: (0)};
    let or_12: order_book_mod::Order= order_book_mod::Order { id: (12),modify: (0),partial:(0), size: (2), price:(0), side:(false), o_type: (0)};
    let or_13: order_book_mod::Order= order_book_mod::Order { id: (13),modify: (0),partial:(0), size: (6), price:(0), side:(false), o_type: (0)};

    // You need some orders in the book
    {
        let mut book = order_book.write().unwrap(); // or `.await` if using `tokio::RwLock`
        book.inserter(or_2);
        book.inserter(or_3);
        book.top_book_refresh();


    }
    

    incoming_orders.push_back(or_1);
    incoming_orders.push_back(or_2);//
    incoming_orders.push_back(or_3);//
    incoming_orders.push_back(or_4);
    incoming_orders.push_back(or_5);
    incoming_orders.push_back(or_6);
    incoming_orders.push_back(or_7);
    incoming_orders.push_back(or_8);
    incoming_orders.push_back(or_9);
/* 
    incoming_orders.push_back(or_10);
    incoming_orders.push_back(or_11);
    incoming_orders.push_back(or_12);
    incoming_orders.push_back(or_13);
*/

    {
        //TRADE!!!
        let mut book = order_book.write().unwrap(); // or `.await` if using `tokio::RwLock`
        book.incoming_orders_processor(&mut incoming_orders, &mut list_matches);
    }
    
    
    
    println!("{:?}",list_matches);
    println!("{:#?}",order_book);

    /*
    // Input
    loop {
        incoming_orders.push_back(Order::new_order());
        order_book.incoming_orders_processor(&mut incoming_orders,&mut list_matches);
        println!("{:#?}",order_book.top_book);
    }
    */
    let shared_data = web::Data::new(order_book.clone());
    //let shared_data = web::Data::new(order_book);
    HttpServer::new(move || {
        
        App::new()
            .route("/", web::post().to(index))
            .app_data(shared_data.clone())
            .service(send_market_data)
            .service(submit_order)     
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

/*    
    // Input
    loop {
        let mut book = order_book.write().unwrap();
        incoming_orders.push_back(Order::new_order());
        book.incoming_orders_processor(&mut incoming_orders,&mut list_matches);
        println!("{:#?}",book.top_book);
    }
*/
    

}
