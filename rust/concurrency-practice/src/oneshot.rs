use std::time::Duration;

use tokio::{sync::oneshot, time::sleep};

// Practice 1 - Basic
// pub async fn run() {
//     let (tx, rx) = oneshot::channel::<i32>();

//     tokio::spawn(async move {
//         sleep(Duration::from_secs(1)).await;
//         match tx.send(42) {
//             Ok(_) => {},
//             Err(_) => {},
//         }
//     });

//     match rx.await {
//         Ok(data) => println!("Data is: {}", data),
//         Err(_) => todo!(),
//     };
// }

// Practice 2 - Request/Response
// #[derive(Debug)]
// struct User {
//     id: u64,
//     name: String,
// }

// pub async fn run() {
//     let (tx, rx) = oneshot::channel::<User>();

//     tokio::spawn(async move {
//         sleep(Duration::from_secs(1)).await;
//         match tx.send(User {
//             id: 1,
//             name: String::from("Mostafa"),
//         }) {
//             Ok(_) => {},
//             Err(_) => {},
//         }
//     });

//     match rx.await {
//         Ok(data) => println!("User is: {:#?}", data),
//         Err(_) => todo!(),
//     };
// }

// Practice 3 - Error
// pub async fn run() {
//     let (tx, rx) = oneshot::channel::<i32>();

//     tokio::spawn(async move {
//         drop(tx);
//     });

//     match rx.await {
//         Ok(data) => println!("Data is: {}", data),
//         Err(error) => println!("Failed to receive: {error}"),
//     };
// }

// Practice 4 - timeout
pub async fn run() {
    let (tx, rx) = oneshot::channel::<i32>();

    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await;
        match tx.send(42) {
            Ok(_) => {},
            Err(_) => {},
        }
    });

    match tokio::time::timeout(Duration::from_secs(1), rx).await {
        Ok(timeout_result) => match timeout_result {
            Ok(data) => println!("Data is: {}", data),
            Err(error) => println!("Failed to receive: {error}"),
        },
        Err(error) => println!("Timeout: {error}"),
    };
}
