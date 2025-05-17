use std::collections::VecDeque;



mod order_book_mod;
mod io_mod;
use order_book_mod::Order;


fn main() {

    let mut order_book = order_book_mod::OrderBook::new();
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

    // test market
    let or_10: order_book_mod::Order= order_book_mod::Order { id: (10),modify: (0),partial:(0), size: (2), price:(0), side:(true), o_type: (0)};
    let or_11: order_book_mod::Order= order_book_mod::Order { id: (11),modify: (0),partial:(0), size: (6), price:(0), side:(true), o_type: (0)};
    let or_12: order_book_mod::Order= order_book_mod::Order { id: (12),modify: (0),partial:(0), size: (2), price:(0), side:(false), o_type: (0)};
    let or_13: order_book_mod::Order= order_book_mod::Order { id: (13),modify: (0),partial:(0), size: (6), price:(0), side:(false), o_type: (0)};

    // You need some orders in the book
    order_book.inserter(or_2);
    order_book.inserter(or_3);
    order_book.top_book_refresh();


    incoming_orders.push_back(or_1);
//    incoming_orders.push_back(or_2);
//    incoming_orders.push_back(or_3);
    incoming_orders.push_back(or_4);
    incoming_orders.push_back(or_5);
    incoming_orders.push_back(or_6);
    incoming_orders.push_back(or_7);
    incoming_orders.push_back(or_8);
    incoming_orders.push_back(or_9);

    incoming_orders.push_back(or_10);
    incoming_orders.push_back(or_11);
    incoming_orders.push_back(or_12);
    incoming_orders.push_back(or_13);

    //TRADE!!!
    order_book.incoming_orders_processor(&mut incoming_orders,&mut list_matches);
    
    
    println!("{:?}",list_matches);
    println!("{:#?}",order_book);

    // Input
    loop {
        incoming_orders.push_back(Order::new_order());
        order_book.incoming_orders_processor(&mut incoming_orders,&mut list_matches);
        println!("{:#?}",order_book.top_book);
    }


    /*
    order_book.inserter(or_1);
    order_book.inserter(or_2);
    order_book.inserter(or_3);
    order_book.inserter(or_4);

    order_book.top_book_refresh();
    println!("TOP: {:?}",order_book.top_book);

    println!("{:?}",order_book);

    println!("Size: {}",order_book.volume_calculator(true, 5));

    println!("Popped order: {:?}",order_book.rem(true,5));
    println!("Size: {}",order_book.volume_calculator(true, 5));
    order_book.top_book_refresh();
    println!("TOP: {:?}",order_book.top_book);

    println!("--------------------------------------------");

    order_book.new_order_handling(or_5);
    order_book.new_order_handling(or_6);
    order_book.new_order_handling(or_7);
    println!("--------------------------------------------");
    println!("{:?}",order_book);
    println!("TOP: {:?}",order_book.top_book);
    order_book.new_order_handling(or_8);
    println!("TOP: {:?}",order_book.top_book);
    println!("{:?}",order_book);
    println!("--------------------------------------------");

    list_matches.append(&mut order_book.new_order_handling(or_9));

    println!("{:#?}",list_matches);
    println!("TOP: {:?}",order_book.top_book);
    println!("{:#?}",order_book);
*/

}
