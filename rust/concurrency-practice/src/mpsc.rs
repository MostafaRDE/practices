use tokio::sync::mpsc;
use tokio::task::JoinHandle;

pub async fn run() {
    let (tx, mut rx) = mpsc::channel::<i32>(10);

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let consumer_handle = tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            println!("Received: {}", data);
        }
    });

    for i in 0..3 {
        let tx = tx.clone();
        handles.push(tokio::spawn(async move {
            for y in 0..3 {
                tx.send(i * 10 + y).await.unwrap();
            }
            drop(tx);
        }));
    }
    drop(tx);

    for handle in handles {
        match handle.await {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    match consumer_handle.await {
        Ok(_) => {}
        Err(_) => {}
    };
}
