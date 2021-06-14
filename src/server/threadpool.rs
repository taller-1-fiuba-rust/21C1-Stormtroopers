use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce(u32) + Send + 'static>;

impl ThreadPool {
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

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(u32) + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} job; executing...", id);

            job(id as u32);

            println!("Worker {} job; ending ...", id);
        });

        Worker { id, thread }
    }
}

#[test]
fn test_new_thread_pool() {
    let threadpool = ThreadPool::new(1);
    let _id = 1;
    threadpool.execute(move |_id| _test_sum());
    assert!(true);
}

fn _test_sum() {
    let _ = 1 + 1;
}
