pub trait Transaction {
    fn commit(&self);
    fn rollback(&self);
}

pub trait TransactionManager<T: Transaction> {
    fn start(&self) -> T;
    fn execute(&self, transaction: T);
    fn fetch<F: FnOnce() -> T>(&self, supplier: F) -> T;
}
