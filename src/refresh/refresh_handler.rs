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

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    //- we should probably just static the damn handler at this point....
    pub fn start(&self) {
        if self.is_running() {
            //panic or something...
            return;
        }

        *self.running.lock().unwrap() = true;

        let tasks = self.tasks.clone();
        let refresh_time = self.refresh_time;
        let running = Arc::clone(&self.running);

        let handle = thread::spawn(move || {
            while *running.lock().unwrap() { //self.is_running()
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
