# Centralized Oracle Design for Bond Pricing Data

## Overview
This document outlines a centralized oracle system designed for bond pricing data. The oracle collects, processes, and delivers detailed bond market information, ensuring accurate and up-to-date data for financial analysis and decision-making.

## Bond Attributes

1. **Bond Prices**:
    - Determined by market trading activity.
    - Calculated as the present value of future cash flows, considering interest rates and time to maturity.

2. **Yield**:
    - The return an investor will receive by holding the bond to maturity.
    - Calculated based on the bond's price, face value, interest rate, and time to maturity.

3. **Maturity Date**:
    - The date on which the bond will mature and the bond issuer will pay the bondholder the face value of the bond.
    - Directly provided by the issuer.

4. **Coupon Rate**:
    - The interest rate the bond issuer will pay on the face value of the bond.
    - Set by the issuer and fixed for the life of the bond.

5. **Credit Rating**:
    - An assessment of the bond issuer's creditworthiness.
    - Obtained from recognized credit rating agencies.

6. **Market Sentiment**:
    - Reflects the market's perception of the bond's value.
    - Derived from trading volumes, news, and financial analysis.

## Data Collection and Processing

- Data sources include financial exchanges, rating agencies, and market news providers.
- Real-time or near-real-time data fetching.
- Processing involves validation, normalization, and integration of diverse data types.

## Contract interface
```rust
    //get base asset the price is reported in
base(e: Env) -> Asset

//get number of decimal places used to represent price for all assets quoted by the oracle
decimals(e: Env) -> u32

//get all assets quoted by the contract
assets(e: Env) -> Vec<Asset>

//get the most recent price update timestamp
last_timestamp(e: Env) -> u64

//get asset price in base asset at specific timestamp
price(e: Env, asset: Asset, timestamp: u64) -> Option<PriceData>

//get the most recent price for an asset
lastprice(e: Env, asset: Asset) -> Option<PriceData>

//get last N price records for the given asset
prices(e: Env, asset: Asset, records: u32) -> Option<Vec<PriceData>>

// Get the maturity date of a specific bond
maturity_date(e: Env, asset: Asset) -> Option<Date>

// Get the coupon rate of a specific bond
coupon_rate(e: Env, asset: Asset) -> Option<Decimal>

// Get the credit rating of a specific bond
credit_rating(e: Env, asset: Asset) -> Option<Rating>

// Get market sentiment for a specific bond
market_sentiment(e: Env, asset: Asset) -> Option<SentimentData>

// Get the yield of a specific bond
bond_yield(e: Env, asset: Asset) -> Option<Decimal>

// Get a list of all bonds quoted by the contract
bonds(e: Env) -> Vec<Asset>

// Get the issuer information of a specific bond
issuer_info(e: Env, asset: Asset) -> Option<IssuerData>

// Get the historical rating changes for a specific bond
rating_history(e: Env, asset: Asset, records: u32) -> Option<Vec<RatingChangeData>>

// Get the historical yield changes for a specific bond
yield_history(e: Env, asset: Asset, records: u32) -> Option<Vec<YieldData>>


//get contract protocol version
version(e: Env) -> u32

//get contract admin address
admin(e: Env) -> Option<Address>
```

### Contract types

```rust
// Existing Definitions
#[contracttype]
enum Asset {
    Stellar(Address),
    Bond(Symbol)
}

#[contracttype]
pub struct PriceData {
    price: i128,
    timestamp: u64
}

#[contracttype]
pub struct Date {
    year: u32,
    month: u32,
    day: u32,
}

#[contracttype]
pub struct Rating {
    rating: u32,
    timestamp: u64
}

#[contracttype]
pub struct SentimentData {
    sentiment: u32,
    timestamp: u64
}

#[contracttype]
pub struct IssuerData {
    issuer: u32,
    timestamp: u64
}

#[contracttype]
pub struct RatingChangeData {
    date: Date,
    rating: Rating,
}

#[contracttype]
pub struct YieldData {
    date: Date,
    yield: i128,
}

```
## Integration with Soroban Contract

1. **Data Push Mechanism**:
    - All the data is pushed every 1 minute.

