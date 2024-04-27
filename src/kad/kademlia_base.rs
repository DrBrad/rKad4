
pub trait KademliaBase {

    fn bind(port: u16);

    fn join();

    fn stop();
}
