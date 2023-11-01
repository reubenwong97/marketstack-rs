use marketstack::api::eod::Eod;
use marketstack::api::exchanges::{self, Exchanges};
use marketstack::api::Query;
use marketstack::{ExchangesData, ExchangesDataItem, ExchangesEodData, Marketstack};

mod setup;

#[test]
#[ignore]
fn test_exchanges() {
    let api_key = setup::setup_key();
    let client = marketstack::Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = Exchanges::builder().limit(25).unwrap().build().unwrap();
    let exchanges_result: ExchangesData = endpoint.query(&client).unwrap();

    assert_eq!(exchanges_result.pagination.limit, 25);
    assert_eq!(exchanges_result.data[0].stock_exchange.mic, "XNAS")
}

#[test]
#[ignore]
fn test_exchanges_mic() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = Exchanges::builder().mic("XNAS").build().unwrap();

    let exchanges_result: ExchangesDataItem = endpoint.query(&client).unwrap();

    assert_eq!(exchanges_result.stock_exchange.mic, "XNAS");
    assert_eq!(exchanges_result.stock_exchange.acronym, "NASDAQ");
    assert_eq!(exchanges_result.stock_exchange.country, "USA");
    assert_eq!(exchanges_result.stock_exchange.country_code, "US");
    assert_eq!(exchanges_result.stock_exchange.city, "New York");
    assert_eq!(exchanges_result.stock_exchange.website, "WWW.NASDAQ.COM");
    assert_eq!(
        exchanges_result.stock_exchange.name,
        "NASDAQ Stock Exchange"
    );
    assert_eq!(exchanges_result.timezone.timezone, "America/New_York");
    assert_eq!(exchanges_result.timezone.abbr, "EST");
    assert_eq!(exchanges_result.timezone.abbr_dst, "EDT");
    assert_eq!(exchanges_result.currency.code, "USD");
    assert_eq!(exchanges_result.currency.name, "US Dollar");
    assert_eq!(exchanges_result.currency.symbol, "$");
}

#[test]
#[ignore]
fn test_exchanges_mic_eod() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = Exchanges::builder()
        .mic("XNAS")
        .eod(Eod::builder().symbol("AAPL").build().unwrap())
        .build()
        .unwrap();

    let exchanges_eod_result: ExchangesEodData = endpoint.query(&client).unwrap();

    assert_eq!(exchanges_eod_result.data.eod[0].symbol, "AAPL")
}
