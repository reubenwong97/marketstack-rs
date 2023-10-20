use marketstack::api::{currencies, AsyncQuery, Query};
use marketstack::{AsyncMarketstack, CurrenciesData, Marketstack};

mod setup;

#[test]
#[ignore]
fn test_currencies() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = currencies::Currencies::builder()
        .limit(3)
        .unwrap()
        .build()
        .unwrap();
    let currencies_result: CurrenciesData = endpoint.query(&client).unwrap();

    assert_eq!(currencies_result.pagination.limit, 3);
    assert_eq!(currencies_result.pagination.offset, 0);

    assert_eq!(currencies_result.data.len(), 3);
}

#[tokio::test]
#[ignore]
async fn test_async_currencies() {
    let api_key = setup::setup_key();
    let client = AsyncMarketstack::new_insecure("api.marketstack.com", api_key)
        .await
        .unwrap();

    let endpoint = currencies::Currencies::builder()
        .limit(3)
        .unwrap()
        .build()
        .unwrap();
    let currencies_result: CurrenciesData = endpoint.query_async(&client).await.unwrap();

    assert_eq!(currencies_result.pagination.limit, 3);
    assert_eq!(currencies_result.pagination.offset, 0);

    assert_eq!(currencies_result.data.len(), 3);
}
