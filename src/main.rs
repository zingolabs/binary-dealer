extern crate tokio;

use tokio::{task::JoinSet, time::sleep, time::Duration};

#[tokio::main]
async fn main() {
    println!("program start!");
    let mut set: JoinSet<()> = JoinSet::new();
    let the_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    for v in the_vec {
        set.spawn(async move {
            println!("started with {:?}", v);
            sleep(Duration::from_secs(2)).await;
            println!("sleep over for {:?}", v);
        });
    }
    println!("about to await the join");
    set.join_all().await;
    println!("program completed, about to exit!");
}
