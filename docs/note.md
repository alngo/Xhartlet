# Clean Architecture

## Domain

### Core concept



Refactor folder hierarchy and think more about model

```
/common
  /value_objects
  /errors

/user
  + id
  + name
  + account
  + portfolio
  + orders

/market
  + id
  + name
  + instruments
  + order_books

/instruments
  + id
  + name
  + history
```

### User
- Account
- Portfolio

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
