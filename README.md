# Rusty_Josh
Single asset exchange usefull to emulate real order book base trading

## Links
### Queue
- https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html#
- https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html#method.pop_front


### Libs
- https://dev.to/fadygrab/learning-rust-15-how-you-can-organize-you-rust-code-with-modules-2c28

### json
- https://github.com/serde-rs/serde?tab=readme-ov-file

### Web API
- https://actix.rs/docs/



## To Do
- [x] Vec deque
- [X] libs
- [X] New order handling (matching func)
- [ ] Check top book refresh on order handling
- [ ] Timestamp
- [ ] Study web API
- [ ] add partial flag to resting order

## Bugs
- the `new_order_handling` function panics when there is no best bid aor ask at the moment that `hread 'main' panicked at src/order_book_mod.rs:226:75:
called `Option::unwrap()` on a `None` value`
    - Create the code safe if there are no orders on the book (even just on one side)


## Ongoing
- Match incoming Order
- Engine return a list of match struct for flexibility
- Testing engine

## Ideas
- Add Best BA to the orderbook struct
- Put in the Order Book struct all the functions to avoid complicating the code with Ownership
- Add a general check in the OrderBook to see if there are some problems

## Road Map
1) Orderbook functionalities: insert, remove quotes, best b/a
2) book visual for debug
3) Matching engine
4) Market data export
5) Json based messages to be globally compatible
6) Local participant (co-location)
7) remote connection (http). 
8) Account manager (unsafe for testing)


# Client

## Virtual Partecipants

### Type of Participants
- **Market Maker** : Liquidity provvider, spread can be influenced by volatility
- **Retail**: Source of noice, small orders, bias against news
- **Shorters**: bias agaist market
- **Big inverstors**: Block trading during the day
- **Profesionals**: Complicated trades

### Bots
- with python
- create subjects (retail, big institutions, market maker) with bias and peculiarities (volume, long short etc)

## Enviroment
- news
- bull or bear market

# Server
- To send the market data first it needs to be made in a format transladable to JSON.
Check the best AB funtion or make a new one

# Matching Engine
- incoming orders
- Check if the new order ID is in the list of processed orders
- Need a vector with struct to keep all matches containing:
    - ask/bid gen info
    - partial fil volume
    - who was the resting order
    - time stamp

- for partial fills ad a letter to the end of ID

```Rust
/// - resting:
///     - true = bid was the resting order
///     - false = ask was the resting order
pub struct Match{
    id_b:,
    id_a:,
    volume_filled:u32,
    price:u32,
    time_stamp:,
    resting: bool
}
```

![Schema ME](/img/New%20order%20Match.png)


## ID
### Format
`123456789-55555-11-22`
1) Order code
2) Partecipant
3) Modification
4) Partial fill


## Time stamps

An Order should have multiple timestamps for:
- Arriving at the Exchange
- Been inserted to the Book
- Full or partial fill
- Modifications
- Cancelations 

This multiple possibilities require a flexible array, a good one can be 
a simple Queue that preserves the order of the events.


```Rust
pub struct Order {
    pub id: u8,
    pub size: u32,
    pub price: f64,
    pub side: bool,
    pub time: VecDeque<TimeStamp>
}
pub struct TimeStamp {
    pub event: u8
    pub time: Timestamp
}
```



![Overview](/img/Overview.png)