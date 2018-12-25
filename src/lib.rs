use std::thread;
pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and save it in the pool
        }

        ThreadPool {
            threads
        }
    }

    pub fn execute<F>(&self, f:F)
        where F: FnOnce() + Send + 'static
    {

    }
}
