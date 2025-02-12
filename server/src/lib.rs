use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    threads: Vec<Thread>,
    sender: mpsc::Sender<Task>,
}

//struct Task;
type Task = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Thread::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { threads, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);

        self.sender.send(task).unwrap();
    }
}

struct Thread {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Thread {
    fn new(
        id: usize, 
        receiver: Arc<Mutex<mpsc::Receiver<Task>>>
    ) -> Thread {
        let thread = thread::spawn(move || {
            while let Ok(task) = receiver
                                    .lock().unwrap().recv() {
                println!("Worker {id} got a task; executing.");
                task();
            }

        });

        Thread { id, thread }
    }
}
