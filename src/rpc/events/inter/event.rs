pub trait Event {

    fn is_prevent_default() -> bool;

    fn prevent_default();
}
