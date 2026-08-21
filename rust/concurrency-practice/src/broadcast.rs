use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};

// Practice 1 - Basic
pub async fn run() {
    let (tx, _) = broadcast::channel::<i32>(10);

    let mut handles: Vec<JoinHandle<()>> = vec![];
    for i in 0..3 {
        let mut rx = tx.subscribe();
        handles.push(tokio::spawn(async move {
            match rx.recv().await {
                Ok(data) => println!("Data of task {i} is {data}"),
                Err(error) => println!("Failed to receive data: {error}"),
            };
        }));
    }

    sleep(Duration::from_secs(1)).await;
    tx.send(40).unwrap();
    tx.send(41).unwrap();
    tx.send(42).unwrap();

    for handle in handles {
        handle.await.unwrap();
    }
}

// Practice 2 - Multiple receivers
// pub async fn run() {
//     let (tx, _) = broadcast::channel::<i32>(10);

//     let mut handles: Vec<JoinHandle<()>> = vec![];
//     for i in 0..3 {
//         let mut rx = tx.subscribe();
//         handles.push(tokio::spawn(async move {
//             while let Ok(data) = rx.recv().await {
//                 println!("Data of task {i} is {data}");
//             };
//         }));
//     }

//     sleep(Duration::from_secs(1)).await;
//     tx.send(40).unwrap();
//     tx.send(41).unwrap();
//     tx.send(42).unwrap();

//     for handle in handles {
//         handle.await.unwrap();
//     }
// }

// Practice 3 - Multiple messages
// pub async fn run() {
//     let (tx, mut rx) = broadcast::channel::<i32>(10);

//     let mut handles: Vec<JoinHandle<()>> = vec![];
//     handles.push(tokio::spawn(async move {
//         loop {
//             match rx.recv().await {
//                 Ok(data) => println!("Data of first task is {data}"),
//                 Err(error) => println!("Failed to receive data: {error}"),
//             }
//         }
//     }));

//     tx.send(1).unwrap();
//     tx.send(2).unwrap();
//     tx.send(3).unwrap();

//     for i in 0..3 {
//         let mut rx = tx.subscribe();
//         handles.push(tokio::spawn(async move {
//             while let Ok(data) = rx.recv().await {
//                 println!("Data of task {i} is {data}");
//             };
//         }));
//     }

//     for handle in handles {
//         handle.await.unwrap();
//     }
// }

// Practice 4 - Lazy subscribe
// pub async fn run() {
//     let (tx, _) = broadcast::channel::<i32>(3);

//     let mut handles: Vec<JoinHandle<()>> = vec![];
//     for i in 0..3 {
//         let mut rx = tx.subscribe();
//         handles.push(tokio::spawn(async move {
//             loop {
//                 match rx.recv().await {
//                     Ok(data) => {
//                         println!("Data of task {i} is {data}");
//                         sleep(Duration::from_secs(1)).await;
//                     }
//                     Err(broadcast::error::RecvError::Lagged(count)) => {
//                         println!("Receiver lagged! Missed {count} messages");
//                     },
//                     Err(error) => {
//                         println!("Task {i} error: {error}");
//                         break;
//                     }
//                 }
//             }
//         }));
//     }

//     tx.send(1).unwrap();
//     tx.send(2).unwrap();
//     tx.send(3).unwrap();
//     tx.send(4).unwrap();
//     tx.send(5).unwrap();

//     for handle in handles {
//         handle.await.unwrap();
//     }
// }

// Practice 5 - Price update
// #[derive(Clone, Debug)]
// struct PriceUpdate {
//     symbol: String,
//     price: i32,
// }

// pub async fn run() {
//     let (tx, _) = broadcast::channel::<PriceUpdate>(3);

//     let mut handles: Vec<JoinHandle<()>> = vec![];
//     for i in 0..4 {
//         let mut rx = tx.subscribe();
//         handles.push(tokio::spawn(async move {
//             loop {
//                 match rx.recv().await {
//                     Ok(data) => {
//                         println!("Data of task {i} is {data:#?}");
//                         sleep(Duration::from_secs(1)).await;
//                     }
//                     Err(broadcast::error::RecvError::Lagged(count)) => {
//                         println!("Receiver lagged! Missed {count} messages");
//                     },
//                     Err(error) => {
//                         println!("Task {i} error: {error}");
//                         break;
//                     }
//                 }
//             }
//         }));
//     }

//     // Publisher
//     tokio::spawn(async move {
//         loop {
//             sleep(Duration::from_secs(1)).await;
//             tx.send(PriceUpdate { symbol: String::from("BTC/USDT"), price: 70_000 }).unwrap();
//         }
//     });

//     for handle in handles {
//         handle.await.unwrap();
//     }
// }

// Practice 6 - Price update with custom subscriber
// #[derive(Clone, Debug)]
// struct PriceUpdate {
//     symbol: String,
//     price: i32,
// }

// async fn logger(mut rx: broadcast::Receiver<PriceUpdate>) {
//     loop {
//         match rx.recv().await {
//             Ok(data) => {
//                 println!("[LOGGER] {} = {}", data.symbol, data.price);
//             },
//             Err(error) => {
//                 println!("Logger error: {error}");
//                 break;
//             }
//         }
//     }
// }

// async fn alert_service(mut rx: broadcast::Receiver<PriceUpdate>) {
//     const THRESHOLD: i32 = 71_000;
//     loop {
//         match rx.recv().await {
//             Ok(data) => {
//                 if data.price > THRESHOLD {
//                     println!("[ALERT] {} crossed {}", data.symbol, data.price);
//                 }
//             },
//             Err(error) => {
//                 println!("Logger error: {error}");
//                 break;
//             }
//         }
//     }
// }

// async fn statistics(mut rx: broadcast::Receiver<PriceUpdate>) {
//     let mut updates_count = 0;
//     loop {
//         match rx.recv().await {
//             Ok(_) => {
//                 updates_count += 1;
//                 println!("Updates count: {updates_count}");
//             },
//             Err(error) => {
//                 println!("Statistics error: {error}");
//                 break;
//             }
//         }
//     }
// }

// async fn slow_consumer(mut rx: broadcast::Receiver<PriceUpdate>) {
//     sleep(Duration::from_secs(3)).await;
//     loop {
//         match rx.recv().await {
//             Ok(_) => {},
//             Err(broadcast::error::RecvError::Lagged(count)) => {
//                 println!("Receiver lagged! Missed {count} messages");
//             },
//             Err(error) => {
//                 println!("Slow-consumer error: {error}");
//                 break;
//             }
//         }
//     }
// }

// pub async fn run() {
//     let (tx, _) = broadcast::channel::<PriceUpdate>(3);

//     let logger_rx = tx.subscribe();
//     let alert_rx = tx.subscribe();
//     let statistics_rx = tx.subscribe();
//     let slow_rx = tx.subscribe();

//     let mut handles: Vec<JoinHandle<()>> = vec![];
//     handles.push(tokio::spawn(logger(logger_rx)));
//     handles.push(tokio::spawn(alert_service(alert_rx)));
//     handles.push(tokio::spawn(statistics(statistics_rx)));
//     handles.push(tokio::spawn(slow_consumer(slow_rx)));

//     // Publisher
//     tokio::spawn(async move {
//         loop {
//             sleep(Duration::from_millis(500)).await;
//             let price = rand::random_range(69000..72000);
//             tx.send(PriceUpdate { symbol: String::from("BTC/USDT"), price }).unwrap();
//         }
//     });

//     for handle in handles {
//         handle.await.unwrap();
//     }
// }
