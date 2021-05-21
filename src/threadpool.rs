use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
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
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} job; executing...", id);

                job();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}

#[test]
fn test_sum(){
    1 + 1;
}
fn test_new_ThreadPool(){
    let threadpool = ThreadPool::new(1);
    threadpool.execute(test_sum );
    assert!(true);
}

/*
pub struct Request {
    pub other: (bool, String),
    pub body: (bool, String),
}

impl Request {
    pub fn new() -> Request {
        let body = (false,String::from(""));
        let other = (false,String::from(""));
        Request {
            other,
            body,
        }
    }
}
 */

