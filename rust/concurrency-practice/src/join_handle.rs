use tokio::time::{Duration, sleep};

// Practice 1
// pub async fn run() {
//     let handle = tokio::spawn(async {
//         42
//     });

//     let result = handle.await;

//     println!("Result: {result:?}");
//     match result {
//         Ok(value) => println!("Value: {value}"),
//         Err(error) => println!("Task failed: {error}"),
//     }
// }

// Practice 2
// pub async fn run() {
//     let handle = tokio::spawn(async {
//         10 + 20 + 30 + 40 + 50
//     });

//     let result = handle.await.unwrap();

//     println!("Result: {result}");
// }

// Practice 3
// pub async fn run() {
//     let mut handles = vec![];

//     for i in 0..5 {
//         handles.push(tokio::spawn(async move {
//             i * 10
//         }));
//     }

//     for handle in handles {
//         match handle.await {
//             Ok(value) => println!("Value: {value}"),
//             Err(error) => println!("Task failed: {error}"),
//         }
//     }
// }

// Practice 4
// pub async fn run() {
//     let mut handles = vec![];

//     handles.push(tokio::spawn(async {
//         sleep(Duration::from_secs(3)).await;
//         println!("Task A");
//     }));

//     handles.push(tokio::spawn(async {
//         sleep(Duration::from_secs(1)).await;
//         println!("Task B");
//     }));

//     handles.push(tokio::spawn(async {
//         sleep(Duration::from_secs(2)).await;
//         println!("Task C");
//     }));

//     for handle in handles {
//         handle.await.unwrap();
//     }
// }

// Practice 5
// pub async fn run() {
//     let handle = tokio::spawn(async {
//         panic!("Something went wrong");
//     });

//     match handle.await {
//         Ok(_) => println!("Success"),
//         Err(error) => println!("Task failed: {error}"),
//     }
// }

// Practice 6
// pub async fn run() {
//     let handle = tokio::spawn(async {
//         Ok::<i32, String>(42)
//     });

//     let result = handle.await;

//     match result {
//         Ok(Ok(value)) => {
//             println!("Success: {value}");
//         },
//         Ok(Err(error)) => {
//             println!("Task returned error: {error}");
//         },
//         Err(join_error) => {
//             println!("Task crashed: {join_error}");
//         },
//     }
// }

// Practice 7
pub async fn run() {
    let mut handles = vec![];

    handles.push(tokio::spawn(async {
        Ok::<i32, String>(1)
    }));

    handles.push(tokio::spawn(async {
        Ok::<i32, String>(2)
    }));

    handles.push(tokio::spawn(async {
        Err::<i32, String>("Cannot calculate value".to_string())
    }));

    handles.push(tokio::spawn(async {
        Ok::<i32, String>(4)
    }));

    handles.push(tokio::spawn(async {
        panic!("Something went wrong")
    }));

    for handle in handles {
        match handle.await {
            Ok(Ok(value)) => println!("Success: {value}"),
            Ok(Err(business_error)) => println!("Business error: {business_error}"),
            Err(handle_error) => eprintln!("Task crashed: {handle_error}")
        }
    }
}
