use std::time::Duration;
use tokio::time::delay_for;

#[tokio::main]
async fn main() {
    for i in 0..10 {
        println!("{}", i);
        delay_for(Duration::from_secs(1)).await;
    }

    let _async_block = async {
        delay_for(Duration::from_secs(1)).await;
        0
    };

    let s = "hoge".to_string();
    let _move_block = async move {
        println!("{}", s);
    };

    //println!("{}", s);
}