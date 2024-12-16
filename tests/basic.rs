use tokio::{
    task::JoinSet,
    time::{sleep, Duration},
};

// testing with tokio::test default
#[tokio::test]
async fn async_singlethread_sanity_check() {
    let mut set: JoinSet<()> = JoinSet::new();
    let the_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    for _v in the_vec {
        set.spawn(async move {
            sleep(Duration::from_millis(200)).await;
        });
    }
    sleep(Duration::from_millis(1000)).await;
    assert!(!set.is_empty());
    assert!(set.len() == 10);
    set.join_all().await;
}

// testing with tokio::test default
#[tokio::test(flavor = "multi_thread")]
async fn async_multithread_sanity_check() {
    let mut set: JoinSet<()> = JoinSet::new();
    let the_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    for _v in the_vec {
        set.spawn(async move {
            sleep(Duration::from_millis(200)).await;
        });
    }
    sleep(Duration::from_millis(1000)).await;
    assert!(!set.is_empty());
    assert!(set.len() == 10);
    set.join_all().await;
}
