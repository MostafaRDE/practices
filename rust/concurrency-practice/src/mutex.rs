use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

pub async fn run() {
    let storage: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..10 {
        let shared = Arc::clone(&storage);
        handles.push(tokio::spawn(async move {
            let value = {
                let mut value = shared.lock().unwrap();
                *value += 1;
                *value
            };
            println!("Data of {} index is: {}", i, value);
        }));
    }

    for handle in handles {
        match handle.await {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    let final_value = storage.lock().unwrap();
    println!("Final value: {}", *final_value);
}

// use tokio::sync::Mutex;

// pub async fn run() {
//     let storage: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

//     let mut handles: Vec<JoinHandle<()>> = Vec::new();

//     for i in 0..10 {
//         let shared = Arc::clone(&storage);
//         handles.push(tokio::spawn(async move {
//             let value = {
//                 let mut value = shared.lock().await;
//                 tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//                 *value += 1;
//                 *value
//             };
//             println!("Data of {} index is: {}", i, value);
//         }));
//     }

//     for handle in handles {
//         match handle.await {
//             Ok(_) => {}
//             Err(_) => {}
//         }
//     }

//     let final_value = storage.lock().await;
//     println!("Final value: {}", *final_value);
// }
