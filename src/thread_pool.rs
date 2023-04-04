/*
Requests that come in are sent to the pool for processing. 
The pool will maintain a queue of incoming requests.
Each of the threads in the pool will pop off a request from this queue, 
handle the request, and then ask the queue for another request. 
With this design, we can process up to N requests concurrently, 
where N is the number of threads. If each thread is responding to a long-running request, 
subsequent requests can still back up in the queue,but we’ve increased the number of 
long-running requests we can handle before reaching that point.

Other options you might explore to improve the throughput of a web server are:
- fork/join model, 
- the single-threaded async I/O model, 
- or the multi-threaded async I/O model. 

(compiler driven development not TDT chapter12)
*/

pub fn pool() ->(mut stream: TcpStream) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}