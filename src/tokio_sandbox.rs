use rand::{self, RngExt};
use std::time::Duration;
use tokio::task;
use tokio::time;

#[tokio::main]
async fn main() {
    spawn_example().await;
}

async fn spawn_example() {
    let join_handler = tokio::task::spawn(random_load(Duration::new(5, 0)));
    let join_handler2 = tokio::task::spawn(random_load(Duration::new(2, 0)));
    let join_handler3 = tokio::task::spawn(random_load(Duration::new(1, 0)));
    let join_handler4 = tokio::task::spawn(random_load(Duration::new(2, 0)));
    join_handler.await.unwrap();
    join_handler2.await.unwrap();
    join_handler3.await.unwrap();
    join_handler4.await.unwrap();
}

async fn random_load(sleep_time: Duration) {
    // println!("From random_load");
    let mut rng: rand::rngs::SmallRng = rand::make_rng();
    let some_random: u32 = rng.random();
    let task_id = tokio::task::id();
    println!("Start sleep: {sleep_time:?} task id: {task_id:?}");
    tokio::time::sleep(sleep_time).await;
    println!("Sleep of task: {task_id:?} end");
    println!("some_random: {some_random:?}");
}
