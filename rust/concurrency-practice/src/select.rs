use tokio::sync::watch;
use tokio::time::{Duration, sleep};

// Practice 1 - First future
pub async fn run() {
    let a = async {
        sleep(Duration::from_secs(1)).await;
        "A"
    };

    let b = async {
        sleep(Duration::from_secs(2)).await;
        "B"
    };

    tokio::select! {
        result = a => {
            println!("Finished: {result}");
        }

        result = b => {
            println!("Finished: {result}");
        }
    }
}

// Practice 2 - Task race
// pub async fn run() {
//     let a = tokio::spawn(async {
//         sleep(Duration::from_secs(1)).await;
//         "A"
//     });

//     let b = tokio::spawn(async {
//         sleep(Duration::from_secs(2)).await;
//         "B"
//     });

//     tokio::select! {
//         result = a => {
//             println!("Finished: {result:?}");
//         }

//         result = b => {
//             println!("Finished: {result:?}");
//         }
//     }
// }

// Practice 3 - Timeout
// pub async fn run() {
//     let operation = async {
//         sleep(Duration::from_secs(5)).await;
//         "Data received"
//     };

//     let timeout = sleep(Duration::from_secs(1));

//     tokio::select! {
//         result = operation => {
//             println!("Operation finished: {result}");
//         }

//         _ = timeout => {
//             println!("Operation timed out");
//         }
//     }
// }

// Practice 4 - Multiple branches
// pub async fn run() {
//     let a = async {
//         sleep(Duration::from_secs(3)).await;
//     };
//     let b = async {
//         sleep(Duration::from_secs(1)).await;
//     };
//     let c = async {
//         sleep(Duration::from_secs(2)).await;
//     };

//     tokio::select! {
//         _ = a => println!("A won"),
//         _ = b => println!("B won"),
//         _ = c => println!("C won"),
//     }
// }

// Practice 5 - Watch and shutdown
// pub async fn run() {
//     let (tx, mut rx) = watch::channel(0);
//     let shutdown = sleep(Duration::from_secs(5));

//     tokio::spawn(async move {
//         tx.send(1).unwrap();
//         sleep(Duration::from_millis(500)).await;

//         tx.send(2).unwrap();
//         sleep(Duration::from_secs(1)).await;

//         tx.send(3).unwrap();
//         sleep(Duration::from_secs(2)).await;

//         tx.send(4).unwrap();
//         sleep(Duration::from_secs(4)).await;
//     });

//     tokio::select! {
//         _ = rx.changed() => {
//             let value = *rx.borrow();
//             println!("Price changed to: {value}");
//         }

//         _ = shutdown => {
//             println!("Shutdown!");
//         }
//     }
// }

// Practice 6 - Worker loop
// pub async fn run() {
//     let (tx, mut rx) = watch::channel(0);

//     tokio::spawn(async move {
//         sleep(Duration::from_millis(500)).await;
//         tx.send(1).unwrap();

//         sleep(Duration::from_secs(1)).await;
//         tx.send(2).unwrap();

//         sleep(Duration::from_secs(2)).await;
//         tx.send(3).unwrap();

//         sleep(Duration::from_secs(4)).await;
//         tx.send(4).unwrap();

//         sleep(Duration::from_secs(8)).await;
//         tx.send(5).unwrap();
//     });

//     loop {
//         let shutdown = sleep(Duration::from_secs(5));
//         tokio::select! {
//             _ = rx.changed() => {
//                 let value = *rx.borrow();
//                 println!("Price changed to: {value}");
//             }

//             _ = shutdown => {
//                 println!("Shutdown!");
//                 break;
//             }
//         }
//     }
// }
