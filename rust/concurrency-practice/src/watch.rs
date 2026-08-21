use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};

// Practice 1 - Basic value
// pub async fn run() {
//     let (tx, rx) = watch::channel(0);
//     println!("Current: {}", *rx.borrow());

//     tx.send(10).unwrap();
//     println!("Current: {}", *rx.borrow());
// }

// Practice 2 - Changed
// pub async fn run() {
//     let (tx, mut rx) = watch::channel(0);
//     tokio::spawn(async move {
//         while rx.changed().await.is_ok() {
//             let value = *rx.borrow();
//             println!("Value: {value}");
//         }
//     });

//     // Model 1
//     // tx.send(1).unwrap();
//     // tx.send(2).unwrap();
//     // tx.send(3).unwrap();
//     // tx.send(4).unwrap();
//     // tx.send(5).unwrap();

//     // Model 2
//     tx.send(1).unwrap();
//     sleep(Duration::from_millis(500)).await;

//     tx.send(2).unwrap();
//     sleep(Duration::from_millis(500)).await;

//     tx.send(3).unwrap();
//     sleep(Duration::from_millis(500)).await;

//     tx.send(4).unwrap();
//     sleep(Duration::from_millis(500)).await;

//     tx.send(5).unwrap();
// }

// Practice 3 - Late receiver
// pub async fn run() {
//     let (tx, mut rx) = watch::channel(0);

//     // Model 1
//     // tx.send(1).unwrap();
//     // tx.send(2).unwrap();
//     // tx.send(3).unwrap();
//     // tx.send(4).unwrap();
//     // tx.send(5).unwrap();

//     // Model 2
//     tx.send(1).unwrap();
//     sleep(Duration::from_millis(500)).await;

//     tx.send(2).unwrap();
//     sleep(Duration::from_millis(500)).await;

//     tx.send(3).unwrap();
//     sleep(Duration::from_millis(500)).await;

//     tx.send(4).unwrap();
//     sleep(Duration::from_millis(500)).await;

//     // tx.send(5).unwrap();


//     tokio::spawn(async move {
//         while rx.changed().await.is_ok() {
//             let value = *rx.borrow();
//             println!("Value: {value}");
//         }
//     });
// }

// Practice 4 - Slow receiver
// pub async fn run() {
//     let (tx, mut rx) = watch::channel(0);

//     tokio::spawn(async move {
//         for value in 1..=10 {
//             tx.send(value).unwrap();
//             println!("[Publisher] Sent: {value}");
//             // sleep(Duration::from_millis(200)).await;
//             sleep(Duration::from_millis(100)).await;
//         }
//     });

//     tokio::spawn(async move {
//         loop {
//             match rx.changed().await {
//                 Ok(_) => {
//                     let value = *rx.borrow();

//                     println!("[Receiver] Received: {value}");

//                     // Intentionally slow
//                     // sleep(Duration::from_secs(1)).await;
//                     sleep(Duration::from_secs(3)).await;
//                 }

//                 Err(_) => {
//                     println!("[Receiver] Channel closed");
//                     break;
//                 }
//             }
//         }
//     });

//     sleep(Duration::from_secs(5)).await;
// }

// Practice 5 - Multiple receivers
#[derive(Clone, Debug)]
struct PriceUpdate {
    symbol: String,
    price: i32,
}

async fn logger(mut rx: watch::Receiver<PriceUpdate>) {
    while rx.changed().await.is_ok() {
        let value = rx.borrow();
        println!("[LOGGER] {} = {}", value.symbol, value.price);
    }
}

async fn monitor(mut rx: watch::Receiver<PriceUpdate>) {
    while rx.changed().await.is_ok() {
        let value = rx.borrow();
        if value.price > 70500 {
            println!("🚨 Price is high!");
        }
    }
}

async fn slow_consumer(mut rx: watch::Receiver<PriceUpdate>) {
    loop {
        match rx.changed().await {
            Ok(_) => {
                let value = rx.borrow();
                println!("[SLOW] {} = {}", value.symbol, value.price);
            }
            Err(error) => {
                println!("Slow-consumer error: {error}");
            }
        }
        sleep(Duration::from_secs(2)).await;
    }
}

async fn current_consumer(mut rx: watch::Receiver<PriceUpdate>) {
    while rx.changed().await.is_ok() {
        let value = rx.borrow();
        println!("[CURRENT] {} = {}", value.symbol, value.price);
    }
}

pub async fn run() {
    let initial_price = PriceUpdate {
        symbol: "BTC/USDT".to_string(),
        price: 70_000,
    };

    let (tx, _) = watch::channel(initial_price);

    let logger_rx = tx.subscribe();
    let monitor_rx = tx.subscribe();
    let slow_rx = tx.subscribe();
    let current_rx = tx.subscribe();

    let mut handles: Vec<JoinHandle<()>> = vec![];
    handles.push(tokio::spawn(logger(logger_rx)));
    handles.push(tokio::spawn(monitor(monitor_rx)));
    handles.push(tokio::spawn(slow_consumer(slow_rx)));
    handles.push(tokio::spawn(current_consumer(current_rx)));

    // Publisher
    tokio::spawn(async move {
        loop {
            let price = rand::random_range(69000..72000);
            tx.send(PriceUpdate { symbol: "BTC/USDT".to_string(), price }).unwrap();
            sleep(Duration::from_millis(500)).await;
        }
    });

    for handle in handles {
        handle.await.unwrap();
    }
}
