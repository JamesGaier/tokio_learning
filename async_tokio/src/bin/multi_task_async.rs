// This only runs one thread and uses co-routines to switch between tasks
// specifically when await is called the task is "paused" and then a different task is run
// until the task is done running the blocking io if any at all
#[tokio::main(flavor = "current_thread")]
async fn main() {
    test_something().await;
}

async fn test_something() {
    std::thread::sleep(std::time::Duration::from_millis(5000));
    println!("Hello from Tokio!");
}
