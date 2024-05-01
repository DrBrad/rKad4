use super::inter::task::Task;

pub struct BucketRefresh {

}

impl BucketRefresh {

}

impl Task for BucketRefresh {

    fn execute(&self) {
        println!("BucketRefresh")
    }
}
