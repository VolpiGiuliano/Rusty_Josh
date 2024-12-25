use std::collections::VecDeque;

use ob_utils::volume_calculator;
mod ob_utils;
mod stru;




fn main() {

    ///////////////////////////
    let mut ask_vecs: Vec<VecDeque<stru::Order>> = vec![VecDeque::new(); stru::ORDER_BOOK_LENGHT]; // Each element is an empty `VecDeque<Order>`
    let mut bid_vecs: Vec<VecDeque<stru::Order>> = vec![VecDeque::new(); stru::ORDER_BOOK_LENGHT];

    // Create references to each `Vec<Order>` for `ask` and `bid`
    let ask_refs: [&mut VecDeque<stru::Order>; stru::ORDER_BOOK_LENGHT] = ask_vecs.iter_mut().collect::<Vec<_>>().try_into().unwrap();
    let bid_refs: [&mut VecDeque<stru::Order>; stru::ORDER_BOOK_LENGHT] = bid_vecs.iter_mut().collect::<Vec<_>>().try_into().unwrap();

    // Initialize the OrderBook struct
    let mut order_book = stru::OrderBook {
        ask: ask_refs,
        bid: bid_refs,
    };
    /////////////////////////////////////////////////////////////

    let or_1: stru::Order= stru::Order { id: (1), size: (4), price:(5.0), side:(true)};
    let or_2: stru::Order= stru::Order { id: (2), size: (3), price:(5.0), side:(true)};
    let or_3: stru::Order= stru::Order { id: (3), size: (3), price:(7.0), side:(false)};
    let or_4: stru::Order= stru::Order { id: (4), size: (10), price:(7.0), side:(false)};

    ob_utils::inserter(or_1, &mut order_book);    
    ob_utils::inserter(or_2, &mut order_book);
    ob_utils::inserter(or_3, &mut order_book);

    ob_utils::inserter(or_4, &mut order_book);

    println!("Size: {}",volume_calculator(true, 5, &mut order_book));

    //println!("Poped order: {:?}",ob_utils::rem(true,3,5,&mut order_book));
    //println!("Size: {}",volume_calculator(true, 5, &mut order_book));
    //println!("{:?}",order_book);

    

    let prov=ob_utils::top_book(stru::ORDER_BOOK_LENGHT, &mut order_book );

    println!("orderBook: {:?}",prov);
    
    println!("Size: {}",volume_calculator(false, 7, &mut order_book));

}
