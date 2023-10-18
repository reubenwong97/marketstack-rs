use marketstack::api::{eod, AsyncQuery, Query};
use marketstack::{AsyncMarketstack, EodData, Marketstack};

mod setup;

#[test]
#[ignore]
fn test_eod() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = eod::Eod::builder().symbol("AAPL").build().unwrap();
    let eod_result: EodData = endpoint.query(&client).unwrap();

    assert_eq!(eod_result.pagination.limit, 100);
    assert_eq!(eod_result.pagination.offset, 0);

    assert_eq!(eod_result.data.len(), 100);
}

#[test]
#[ignore]
fn test_eod_paged() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = eod::Eod::builder()
        .symbol("AAPL")
        .limit(5)
        .unwrap()
        .build()
        .unwrap();
    let eod_result: EodData = endpoint.query(&client).unwrap();

    assert_eq!(eod_result.pagination.limit, 5);
    assert_eq!(eod_result.data.len(), 5);
}

#[tokio::test]
#[ignore]
async fn test_async_eod() {
    let api_key = setup::setup_key();
    let client = AsyncMarketstack::new_insecure("api.marketstack.com", api_key)
        .await
        .unwrap();

    let endpoint = eod::Eod::builder().symbol("AAPL").build().unwrap();
    let eod_result: EodData = endpoint.query_async(&client).await.unwrap();

    assert_eq!(eod_result.pagination.limit, 100);
    assert_eq!(eod_result.pagination.offset, 0);

    assert_eq!(eod_result.data.len(), 100);
}

#[tokio::test]
#[ignore]
async fn test_async_eod_paged() {
    let api_key = setup::setup_key();
    let client = AsyncMarketstack::new_insecure("api.marketstack.com", api_key)
        .await
        .unwrap();

    let endpoint = eod::Eod::builder()
        .symbol("AAPL")
        .limit(5)
        .unwrap()
        .build()
        .unwrap();
    let eod_result: EodData = endpoint.query_async(&client).await.unwrap();

    assert_eq!(eod_result.pagination.limit, 5);
    assert_eq!(eod_result.data.len(), 5);
}
