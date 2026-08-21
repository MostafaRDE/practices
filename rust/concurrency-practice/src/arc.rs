use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn run() {
    let storage: Arc<i32> = Arc::new(0);

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..10 {
        let shared = Arc::clone(&storage);
        handles.push(tokio::spawn(async move {
            println!("Data of {} index is: {}", i, shared);
        }));
    }

    for handle in handles {
        match handle.await {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
