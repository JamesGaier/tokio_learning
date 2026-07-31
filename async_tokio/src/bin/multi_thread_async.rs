
// multiple threads (up to 10) 
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    // puase the test_something task while the io blocks and do other tasks in the meantime
    test_something().await;
}

// an async function that waits 5 seconds
async fn test_something() {
    std::thread::sleep(std::time::Duration::from_millis(5000));
    println!("Hello from Tokio!");
}
