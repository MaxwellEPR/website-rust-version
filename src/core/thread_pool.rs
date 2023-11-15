use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    size: usize,
    workers: Arc<Vec<Worker>>,
    task_queue: Arc<Mutex<Vec<Job>>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workder = Vec::with_capacity(size);
        let task_queue = Arc::new(Mutex::new(Vec::new()));

        for i in 0..size {
            workder.push(Worker::new(i,task_queue.clone()));
        }

        ThreadPool {
            size,
            workers:Arc::new(workder),
            task_queue,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.task_queue.lock().unwrap().push(Box::new(f));
    }
}

impl Clone for ThreadPool {
    fn clone(&self) -> Self {
        ThreadPool {
            size: self.size,
            workers: self.workers.clone(),
            task_queue: self.task_queue.clone(),
        }
    }
}

pub struct Worker {
    id: usize,
    thread: JoinHandle<Job>,
}

impl Worker {
    pub fn new(id: usize, task_queue: Arc<Mutex<Vec<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            if let Some(fun) = task_queue.lock().unwrap().pop() {
                fun();
            }
        });
        Worker { id, thread }
    }
}

#[cfg(test)]
mod test {
    use super::ThreadPool;

    #[test]
    pub fn test_thread_pool() {
        let thread_pool = ThreadPool::new(2);
        thread_pool.execute(Box::new(move ||{
            println!("aaaa");
        }));
        thread_pool.execute(Box::new(move ||{
            println!("bbbb");
        }));

    }
}
