use marketstack::api::{timezones, AsyncQuery, Query};
use marketstack::{AsyncMarketstack, Marketstack, TimezonesData};

mod setup;

#[test]
#[ignore]
fn test_timezones() {
    let api_key = setup::setup_key();
    let client = Marketstack::new_insecure("api.marketstack.com", api_key).unwrap();

    let endpoint = timezones::Timezones::builder()
        .limit(3)
        .unwrap()
        .build()
        .unwrap();
    let timezones_result: TimezonesData = endpoint.query(&client).unwrap();

    assert_eq!(timezones_result.pagination.limit, 3);
    assert_eq!(timezones_result.pagination.offset, 0);

    assert_eq!(timezones_result.data.len(), 3);
}

#[tokio::test]
#[ignore]
async fn test_async_timezones() {
    let api_key = setup::setup_key();
    let client = AsyncMarketstack::new_insecure("api.marketstack.com", api_key)
        .await
        .unwrap();

    let endpoint = timezones::Timezones::builder()
        .limit(3)
        .unwrap()
        .build()
        .unwrap();
    let timezones_result: TimezonesData = endpoint.query_async(&client).await.unwrap();

    assert_eq!(timezones_result.pagination.limit, 3);
    assert_eq!(timezones_result.pagination.offset, 0);

    assert_eq!(timezones_result.data.len(), 3);
}
