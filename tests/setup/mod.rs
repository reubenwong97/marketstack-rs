use dotenvy::dotenv;
use std::env;

pub fn setup_key() -> String {
    dotenv().expect(".env file does not exist");

    env::var("MARKETSTACK_API_KEY").expect("MARKETSTACK_API_KEY must be set.")
}
