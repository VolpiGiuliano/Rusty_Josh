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



## To Do
- [x] Vec deque
- [X] libs
- [ ] option `<struct>` return
- [ ] New order handling (matching func)
- [ ] Timestamp


## Road Map
1) Orderbook functionalities: insert, remove quotes, best b/a
2) book visual for debug
3) Matching engine
4) Market data export
5) Json based messages to be globally compatible
6) Local participant (co-location)
7) remote connection (http). 
8) Account manager (unsafe for testing)


## Data Visual
- Use python? The rich library can do a clean app for the terminal

# Virtual Partecipants

## Bots
- with python
- create subjects (retail, big institutions, market maker) with bias and peculiarities (volume, long short etc)

## Enviroment
- news
- bull or bear market

## Type of Participants
- **Market Maker** : Liquidity provvider, spread can be influenced by volatility
- **Retail**: Source of noice, small orders, bias against news
- **Shorters**: bias agaist market
- **Big inverstors**: Block trading during the day
- **Profesionals**: Complicated trades
- [X] test best orders
- [ ] add cross in best
- [ ] test cross
- [ ] add time id to know the order usefull for cross

# Matching Engine

![Schema ME](/img/New%20order%20Match.png)

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