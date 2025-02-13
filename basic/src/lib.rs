use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    threads: Vec<Thread>,
    sender: Option<mpsc::Sender<Task>>,
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

        ThreadPool { 
            threads, 
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);

        self.sender
            .as_ref().unwrap()
            .send(task).unwrap();
    }
}

// need drop for ya types bruh
// especially for threads
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for thread in &mut self.threads {
            println!{"Shutting down thread {}", thread.id};
            
            if let Some(thread) = thread.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// ya either got Some or ya got None
struct Thread {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Thread {
    fn new(
        id: usize, 
        receiver: Arc<Mutex<mpsc::Receiver<Task>>>
    ) -> Thread {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(task) => {
                    println!("Thread {id} got a task; executing.");

                    task();
                }
                Err(_) => {
                    println!("Thread {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Thread { 
            id, 
            thread: Some(thread),
        }
    }
}
