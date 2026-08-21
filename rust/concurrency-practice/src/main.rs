mod arc;
mod mutex;
mod rwlock;
mod mpsc;
mod oneshot;
mod broadcast;

#[tokio::main]
async fn main() {
    println!("Run Arc file:");
    arc::run().await;
    println!("\n\n");

    println!("Run Mutex file:");
    mutex::run().await;
    println!("\n\n");

    println!("Run RwLock file:");
    rwlock::run().await;
    println!("\n\n");

    println!("Run MPSC file:");
    mpsc::run().await;
    println!("\n\n");

    println!("Run Oneshot file:");
    oneshot::run().await;
    println!("\n\n");

    println!("Run Broadcast file:");
    broadcast::run().await;
    println!("\n\n");
}
