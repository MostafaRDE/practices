use tokio::sync::{mpsc, watch};
use tokio::time::{Duration, sleep};

// Practice 1 - Abort task
// pub async fn run() {
//     let handle = tokio::spawn(async {
//         println!("Working...");
//         sleep(Duration::from_secs(1)).await;
//         println!("Worker finished");
//     });

//     sleep(Duration::from_millis(100)).await;
//     handle.abort();
// }

// Practice 2 - Cancel with watch
// Practice 3 - Select work and shutdown
// pub async fn run() {
//     let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

//     tokio::spawn(async move {
//         sleep(Duration::from_secs(7)).await;
//         shutdown_tx.send(true).unwrap();
//     });

//     loop {
//         let do_work = sleep(Duration::from_secs(2));

//         tokio::select! {
//             _ = shutdown_rx.changed() => {
//                 if *shutdown_rx.borrow() {
//                     println!("Shutdown requested");
//                     break;
//                 }
//             }

//             _ = do_work => {
//                 println!("Work completed");
//             }
//         }
//     }
// }

// Practice 4 - Graceful shutdown
// pub async fn run() {
//     let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

//     let shutdown_loop_rx = shutdown_tx.subscribe();

//     tokio::spawn(async move {
//         sleep(Duration::from_secs(7)).await;
//         shutdown_tx.send(true).unwrap();
//     });

//     tokio::spawn(async move {
//         if shutdown_rx.changed().await.is_ok() {
//             let value = *shutdown_rx.borrow();
//             if value {
//                 println!("Shutdown requested");
//                 println!("Finishing. current works");
//             }
//         };
//     });

//     loop {
//         if *shutdown_loop_rx.borrow() {

//             println!("No more work");
//             break;
//         }

//         let do_work = sleep(Duration::from_secs(2));

//         do_work.await;

//         println!("Work completed");
//     }
// }

// Practice 5 - Worker shutdown
#[derive(Debug)]
struct Job {
    id: u64,
    payload: String,
}

async fn producer(tx: mpsc::Sender<Job>, producer_id: u64, mut shutdown_rx: watch::Receiver<bool>) {
    for i in 0..20 {
        tokio::select! {
            result = tx.send(Job {
                id: producer_id * 100 + i,
                payload: format!("Job from producer {producer_id}"),
            }) => {
                result.unwrap()
            }

            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    println!("Producer {producer_id} stopping");
                    break;
                }
            }
        }
    }
}

async fn do_work(job: &Job) {
    println!("Processing job: {job:?}");

    sleep(Duration::from_secs(2)).await;

    println!("Completed job: {}", job.id);
}

pub async fn run() {
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

    let (tx, mut rx) = mpsc::channel::<Job>(100);

    let mut producer_handles = vec![];

    let consumer_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(job) = rx.recv() => {
                    do_work(&job).await;
                }

                _ = shutdown_rx.changed() => {
                    let value = *shutdown_rx.borrow();
                    if value {
                        println!("Shutdown requested");
                        break;
                    }
                }
            }
        }

        println!("Draining remaining jobs...");

        // Graceful shutdown
        while let Some(job) = rx.recv().await {
            do_work(&job).await;
        }

        println!("Queue drained. Worker stopped.");
    });

    for i in 0..3 {
        producer_handles.push(tokio::spawn(producer(tx.clone(), i, shutdown_tx.subscribe())));
    }

    drop(tx);

    sleep(Duration::from_secs(9)).await;
    shutdown_tx.send(true).unwrap();

    consumer_handle.await.unwrap();
}
