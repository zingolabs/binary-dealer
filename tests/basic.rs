use tokio::{
    task::JoinSet,
    time::{sleep, Duration},
};

#[tokio::test(flavor = "multi_thread")]
async fn async_multitask_sanity_check() {
    let mut set: JoinSet<()> = JoinSet::new();
    let the_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    for _v in the_vec {
        set.spawn(async move {
            sleep(Duration::from_millis(200)).await;
        });
    }
    sleep(Duration::from_millis(2000)).await;
    assert!(!set.is_empty());
    assert!(set.len() == 10);
    set.join_all().await;
}
