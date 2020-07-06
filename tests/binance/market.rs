use openlimits::binance::Binance;

#[tokio::test]
async fn get_depth() {
    let exchange = Binance::new();
    let resp = exchange.get_depth("BNBBTC", 50).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_prices() {
    let exchange = Binance::new();
    let resp = exchange.get_all_prices().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_price() {
    let exchange = Binance::new();
    let resp = exchange.get_price("BNBBTC").await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_book_tickers() {
    let exchange = Binance::new();
    let resp = exchange.get_all_book_tickers().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_book_ticker() {
    let exchange = Binance::new();
    let resp = exchange.get_book_ticker("BNBBTC").await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_24h_price_stats() {
    let exchange = Binance::new();
    let resp = exchange.get_24h_price_stats("BNBBTC").await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_klines() {
    let exchange = Binance::new();
    let resp = exchange
        .get_klines("BNBBTC", "5m", 2, None, None)
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_24h_price_stats_all() {
    let exchange = Binance::new();
    let resp = exchange.get_24h_price_stats_all().await.unwrap();
    println!("{:?}", resp);
}
