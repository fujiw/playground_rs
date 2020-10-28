use std::time::Duration;

#[tokio::main]
async fn main() {
    for i in 0..10 {
        println!("{}", i);
        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}