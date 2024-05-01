use crate::refresh::tasks::inter::task::Task;
//use std::old_io::Timer;

pub struct RefreshHandler {
    tasks: Vec<Box<dyn Task>>
}

impl RefreshHandler {

    pub fn new() -> Self {
        Self {
            tasks: Vec::new()
        }
    }

    pub fn start(&self) {
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
