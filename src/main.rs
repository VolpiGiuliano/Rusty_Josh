#[derive(Clone)]
#[derive(Debug)]
struct Order{
    id:u8,
    size:u32
}


#[derive(Debug)]
struct OrderBook<'oblt>{
    ask: [&'oblt mut Vec<Order>;10],
    bid: [&'oblt mut Vec<Order>;10]
}



fn main() {

    ///////////////////////////
    let mut ask_vecs: Vec<Vec<Order>> = vec![vec![]; 10]; // Each element is an empty `Vec<Order>`
    let mut bid_vecs: Vec<Vec<Order>> = vec![vec![]; 10];

    // Create references to each `Vec<Order>` for `ask` and `bid`
    let ask_refs: [&mut Vec<Order>; 10] = ask_vecs.iter_mut().collect::<Vec<_>>().try_into().unwrap();
    let bid_refs: [&mut Vec<Order>; 10] = bid_vecs.iter_mut().collect::<Vec<_>>().try_into().unwrap();

    // Initialize the OrderBook struct
    let order_book = OrderBook {
        ask: ask_refs,
        bid: bid_refs,
    };
    
    // inserter 
    let or_1: Order= Order { id: (1), size: (3) };
    let or_2: Order= Order { id: (2), size: (3) };
    order_book.ask[0].push(or_1);
    order_book.ask[0].push(or_2);
    println!("{:?}",*order_book.ask[0]);
    println!("{:?}",order_book.ask[0].pop());
    println!("{:?}",*order_book.ask[0]);
    
}
