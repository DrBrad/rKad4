use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use crate::refresh::tasks::inter::task::Task;
//use std::old_io::Timer;

pub struct RefreshHandler {
    tasks: Vec<Box<dyn Task>>,//Vec<Box<dyn Task>>,
    refresh_time: u64,
    running: Arc<Mutex<bool>>
}

impl RefreshHandler {

    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            refresh_time: 1000,//3600000
            running: Arc::new(Mutex::new(false))
        }
    }

    pub fn is_running() -> bool {
        false
    }

    pub fn start(&self) {
        let tasks = self.tasks.clone();
        let refresh_time = self.refresh_time;
        *self.running.lock().unwrap() = true;
        let running = Arc::clone(&self.running);

        let handle = thread::spawn(move || {
            while *running.lock().unwrap() {
                for task in &tasks {
                    task.execute();
                }

                sleep(Duration::from_millis(refresh_time));
            }
        });

        //handle.join().unwrap();
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    pub fn add_operation(&mut self, task: Box<dyn Task>) {
        self.tasks.push(task);
    }
}