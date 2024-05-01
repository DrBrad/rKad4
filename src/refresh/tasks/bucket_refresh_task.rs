use super::inter::task::Task;

#[derive(Clone)]
pub struct BucketRefreshTask {

}

impl BucketRefreshTask {

    pub fn new() -> Self {
        Self {

        }
    }
}

impl Task for BucketRefreshTask {

    fn execute(&self) {
        println!("BucketRefresh");
    }

    fn clone_dyn(&self) -> Box<dyn Task> {
        Box::new(self.clone())
    }
}
