
pub trait KademliaBase {

    fn bind(&self, port: u16);

    fn join(&self);

    fn stop(&self);
}
