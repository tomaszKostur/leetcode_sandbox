use rand::{self, RngExt};
use std::time::Duration;
use tokio::task;
use tokio::time;
use tokio::sync::RwLock;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // spawn_example().await;
    random_write_example().await;
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


async fn random_write_example(){
    let mut shared_vec: Vec<u32> = vec![1,2,3];
    let sv_lock = Arc::new(RwLock::new(shared_vec));

    let jh1 = tokio::task::spawn(
        random_write(sv_lock.clone())
    );

    jh1.await.unwrap();
}

async fn random_write(rwlock: Arc<RwLock<Vec<u32>>>) {
    let task_id = tokio::task::id();
    let mut rng: rand::rngs::SmallRng = rand::make_rng();
    let some_random: u32 = rng.random();
    // let resource = rwlock.read().await; // WARNING: If you do not put it in a scope then you'll have a deadlock!;
    let mut w_resource = rwlock.write().await;
    w_resource.push(6);
    println!("Resource from task: {task_id}, {w_resource:?}");
}