mod order_book_mod;



fn main() {


    let mut order_book = order_book_mod::OrderBook::new();


    let or_1: order_book_mod::Order= order_book_mod::Order { id: (1), size: (4), price:(5.0), side:(true)};
    let or_2: order_book_mod::Order= order_book_mod::Order { id: (2), size: (3), price:(5.0), side:(true)};
    let or_3: order_book_mod::Order= order_book_mod::Order { id: (3), size: (3), price:(7.0), side:(false)};
    let or_4: order_book_mod::Order= order_book_mod::Order { id: (4), size: (10), price:(7.0), side:(false)};
    let or_5: order_book_mod::Order= order_book_mod::Order { id: (5), size: (1), price:(8.0), side:(false)};
    let or_6: order_book_mod::Order= order_book_mod::Order { id: (6), size: (1), price:(4.0), side:(true)};
    let or_7: order_book_mod::Order= order_book_mod::Order { id: (7), size: (2), price:(3.0), side:(false)};
    let or_8: order_book_mod::Order= order_book_mod::Order { id: (8), size: (2), price:(9.0), side:(true)};





    order_book.inserter(or_1);
    order_book.inserter(or_2);
    order_book.inserter(or_3);
    order_book.inserter(or_4);


    println!("{:?}",order_book);

    println!("Size: {}",order_book.volume_calculator(true, 5));

    println!("Poped order: {:?}",order_book.rem(true,3,5));
    println!("Size: {}",order_book.volume_calculator(true, 5));
    
    println!("TOP: {:?}",order_book.top_book());

}
