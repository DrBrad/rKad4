use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use crate::refresh::tasks::inter::task::Task;
//use std::old_io::Timer;

pub struct RefreshHandler {
    tasks: Vec<Box<dyn Task>>,//Vec<Box<dyn Task>>,
    refresh_time: u64
}

impl RefreshHandler {

    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            refresh_time: 3600000
        }
    }

    pub fn is_running() -> bool {
        false
    }

    pub fn start(&'static mut self) {
        //let tasks = self.tasks.clone(); // Clone the tasks vector
        //let tasks = self.tasks.iter().clone();//Arc::clone(&self.tasks);
        let refresh_time = self.refresh_time;

        let handle = thread::spawn(move || {

            /*
            for task in &tasks {
                task.execute();
            }
            */

            sleep(Duration::from_millis(refresh_time));
        });

        handle.join().unwrap();

        /*
        let mut timer = Timer::new().unwrap();
        let ticks = timer.periodic(Duration::minutes(5));
        for _ in ticks.iter() {
            your_function();
        }
        */
    }

    pub fn stop(&self) {

    }

    pub fn add_operation(&mut self, task: Box<dyn Task>) {
        self.tasks.push(task);
    }
}
