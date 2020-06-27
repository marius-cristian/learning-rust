#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("doing some work, asynchronously");

        // Return a value for the example
        "result of the computation"
    });

    // Wait for the spawned task to finish
    let res = handle.await;

    println!("got {:?}", res);
}