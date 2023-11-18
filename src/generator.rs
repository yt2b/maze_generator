pub trait Generator {
    fn in_process(&self) -> bool;
    fn proceed(&mut self);
    fn get_field(&self) -> &Vec<Vec<bool>>;
}
