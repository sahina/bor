/// Represents an operation that accepts a single input argument and returns no result.
pub trait Consumer<T> {
    ///  Performs this operation on the given argument.
    fn accept(self, value: T) -> Self;

    /// Returns a composed `Consumer` that performs, in sequence, this
    /// operation followed by the `after` operation. If performing either
    /// operation throws an exception, it is relayed to the caller of the
    /// composed operation. If performing this operation throws an exception,
    /// the `after` operation will not be performed.
    fn and_then<N: Consumer<T>>(&self, after: N) -> N {
        after
    }
}
