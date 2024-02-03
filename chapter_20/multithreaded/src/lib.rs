use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// pub struct ThreadPool {
//     threads: Vec<thread::JoinHandle<()>>,
// }

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // we could change new into build and return a Result
    //But we’ve decided in this case that trying to create a thread pool without any threads should
    //be an unrecoverable error.
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {}

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // let thread = thread::spawn(|| {
        //     receiver;
        // });
        let thread = thread::spawn(move || loop {
            // acquiring a lock might fail if the mutex is in a poisoned state, which can happen if
            // some other thread panicked while holding the lock rather than releasing the lock.
            let job = {
                let lock = receiver.lock().unwrap();
                println!("worker {id} lock the Mutex.");
                lock.recv().unwrap()
            };

            println!("worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
