use dotenvy::dotenv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(any(test, debug_assertions)) {
        dotenv().expect("can't load .env file");
    }
    Ok(())
}
