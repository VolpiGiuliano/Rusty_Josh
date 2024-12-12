use std::collections::VecDeque;
mod ob_utils;

#[derive(Clone)]
#[derive(Debug)]
/// - Side: Bid=true - Ask=false
struct Order{
    id:u8,
    size:u32,
    price:f64,
    side: bool
}


#[derive(Debug)]
struct OrderBook<'oblt>{
    ask: [&'oblt mut VecDeque<Order>;10],
    bid: [&'oblt mut VecDeque<Order>;10]
}

static ORDER_BOOK_LENGHT: usize=10;




fn main() {

    ///////////////////////////
    let mut ask_vecs: Vec<VecDeque<Order>> = vec![VecDeque::new(); ORDER_BOOK_LENGHT]; // Each element is an empty `VecDeque<Order>`
    let mut bid_vecs: Vec<VecDeque<Order>> = vec![VecDeque::new(); ORDER_BOOK_LENGHT];

    // Create references to each `Vec<Order>` for `ask` and `bid`
    let ask_refs: [&mut VecDeque<Order>; ORDER_BOOK_LENGHT] = ask_vecs.iter_mut().collect::<Vec<_>>().try_into().unwrap();
    let bid_refs: [&mut VecDeque<Order>; ORDER_BOOK_LENGHT] = bid_vecs.iter_mut().collect::<Vec<_>>().try_into().unwrap();

    // Initialize the OrderBook struct
    let mut order_book = OrderBook {
        ask: ask_refs,
        bid: bid_refs,
    };
    /////////////////////////////////////////////////////////////

    let or_1: Order= Order { id: (1), size: (3), price:(5.0), side:(true)};
    let or_2: Order= Order { id: (2), size: (3), price:(5.0), side:(true)};
    let or_3: Order= Order { id: (2), size: (3), price:(7.0), side:(false)};

    ob_utils::inserter(or_1, &mut order_book);
    println!("{:?}",order_book);
    
    ob_utils::inserter(or_2, &mut order_book);
    println!("{:?}",order_book);

    println!("Poped order: {:?}",ob_utils::rem(true,3,5,&mut order_book));
    println!("{:?}",order_book);

    ob_utils::inserter(or_3, &mut order_book);
    println!("{:?}",order_book);


}
