use std::{sync::{mpsc, Arc, Mutex}, thread};



pub struct ThreadPool{
    workers:  Vec<Worker>,
    sender: Option< mpsc::Sender<Job>>
}


impl ThreadPool {
    pub fn new(size: usize) ->  Self {
        assert!(size >  0);

        let mut workers =  Vec::with_capacity(size);
        let (tx, rx) =  mpsc::channel();

        let rx =  Arc::new(Mutex::new(rx));

        for n in 0..size{
            workers.push(Worker::new(n, Arc::clone(&rx)));
        }

        ThreadPool{
            workers,
            sender:Some(tx)
        }
    }

    /// Create a new ThreadPool
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function  will panic if the size is zero
    pub fn execute<F>(&self,  f:F)
    where
        F: FnOnce() +  Send + 'static
    {
        let job =  Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl  Drop for ThreadPool{
    fn drop(&mut self) {
        for worker in &mut self.workers {
            drop(self.sender.take());
            println!("Shutting down worker {}", worker.id);

            if let Some(thread_) = worker.thread.take() {
                thread_.join().unwrap();
            }
        }
    }
}

struct Worker{
    id:  usize,
    thread:  Option<thread::JoinHandle<()>>
}

impl Worker{
    fn new(id:usize,  receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self{

        let handler =  thread::spawn( move || loop{
            let message =  receiver.lock().unwrap().recv();

            match message {
                Ok(job) =>{
                    println!("Worker {id} got a job;  executing.");
                    job();
                },
                Err(_) =>{
                    println!("Worker  {id} disconnected;  shutting down");
                    break;
                }
            }


        });
        Worker{
            id,
            thread:Some(handler)
        }
    }
}

type Job =  Box<dyn FnOnce() + Send +  'static>;