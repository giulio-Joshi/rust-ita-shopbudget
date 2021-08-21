# Resolution of Budged Electronic Shop code Kata

https://github.com/rust-italia/katas/tree/master/electronics-shop


## Known weaknesess

* Probably memory~intense because uses `Vec<u64>` and no reference to immutable data.
* Prioritize keyboard value over USB driver value, might discard best cases when a lot of items are available in the shop.