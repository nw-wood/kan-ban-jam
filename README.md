# kan-ban-jam

//notes about asyncronous operations
//use async when you need I/O or long-running tasks
//don't block the thread in async functions
//keep async functions small

//use await for async tasks, but limit its usage

    /*
        async fn fetch_data() {
            let data = async_get_data().await;  // Good
            process_data(data);  // Processing happens after awaiting
        }
    */

//handle async errors explcitly - when writing async functions return results<t, e>'s as good practice, and handle errors from them

    /*
        async fn fetch_data() -> Result<String, SomeError> {
            let response = http_call().await?;
            Ok(response)
        }
    */

//be careful with data with a shared state, like data contained in an arc mutex, or a tokio mutex
    /*
        let shared_data = Arc::new(Mutex::new(vec![])); <--- careful handling this, do futures hold locks? IDK, maybe, but during execution via awaiting them they certainly would
    */

//avoid using async in constructors (like associated new functions)

    /*
        impl MyStruct {
            fn new() -> Self { /* synchronous */ }
            async fn initialize(&mut self) { /* async setup */ }
        }
    */

//use tokio::spawn for parallel execution

    /*
        let handle1 = tokio::spawn(async_task1()); <-------- future for task is created and bound to handle1
        let handle2 = tokio::spawn(async_task2());

        let result1 = handle1.await?;  <-------------------- the future is executed now for handle1, but just as well if handle2.await?; was called then out of order exec would have been fine
        let result2 = handle2.await?;
    */

//understand that async functions return futures *

    /*
        async fn example() -> u32 {
            42
        }

        let future = example();  // Returns a Future, but doesn't run
        let result = future.await;  // Now the future runs

        #[derive(Serialize, Deserialize, Debug)]
        struct Board {
            name: String,
            items: Vec<Item>,
            statuses: Vec<String>,
        }
    */

//use timeouts instead of sleeping threads and stuff - tokio has a way of executing a future after a duration of time without breaking things

    //let result = tokio::time::timeout(Duration::from_secs(5), async_task()).await;