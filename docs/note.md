# Clean Architecture

## Domain

### Core concept



Refactor folder hierarchy and think more about model
Simplify

```
/common
  /value_objects
  /errors

/user -> root aggregate
  + id
  + name
  + account
  + portfolio -> aggregate
    + positions

/ticker
  + id
  + history
    + candles
  + orders
```

### ValueObject

- [x] Id,
- [x] Price
- [x] Quantity
- [x] Ticker
- [x] Timeframe
- [x] Direction
- [x] Order Kind
- [x] Order Status

### Aggregate

- [x] User
- [x] Account
- [x] Portfolio

- [x] Market
- [x] History
- [x] OrderBook
- [x] Order
- [x] Position
- [x] Instrument
- [x] Candle

### Services
