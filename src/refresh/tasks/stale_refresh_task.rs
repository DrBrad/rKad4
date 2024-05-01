use super::inter::task::Task;

#[derive(Clone)]
pub struct StaleRefreshTask {

}

impl StaleRefreshTask {

    pub fn new() -> Self {
        Self {

        }
    }
}

impl Task for StaleRefreshTask {

    fn execute(&self) {
        println!("StaleRefresh");
    }

    fn clone_dyn(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}
