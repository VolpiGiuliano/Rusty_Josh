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

## Time stamp


## To Do
- [x] Vec deque
- [X] libs
- [ ] option <struct> return
- [X] test best orders
- [ ] add cross in best
- [ ] test cross
- [ ] add time id to know the order usefull for cross

## Road Map
1) Orderbook functionalities: insert, remove quotes, best b/a
2) book visual for debug
3) Matching engine
4) Market data export
5) Local participant (co-location). Json based messages to be globally compatible
6) remote connection (http). 
7) Account manager (unsafe for testing)


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
