
pub trait Clock {
    type Error;
    fn clock(&mut self) -> Result<(), Self::Error>;
}
