use crate::ddd::aggregate::Aggregate;
use crate::tester::framework::when;
use crate::tester::validator::AggregateResultValidator;

/// Holds the initial event state of an aggregate and accepts a command.
pub struct AggregateTestExecutor<A>
where
    A: Aggregate,
{
    pub(crate) events: Vec<A::Event>,
    pub(crate) service: A::Service,
}

impl<A> AggregateTestExecutor<A>
where
    A: Aggregate,
{
    /// Consumes a command and using the state details previously passed provides a validator object
    /// to test against.
    ///
    /// ```
    /// use base::test::framework::TestFramework;
    ///
    /// let executor = TestFramework::<MyAggregate>::with(MyService)
    ///     .given_no_previous_events();
    ///
    /// let validator = executor.when(MyCommands::DoSomething);
    /// ```
    pub fn when(self, command: A::Command) -> AggregateResultValidator<A> {
        let result = when::<A>(self.events, command, self.service);
        AggregateResultValidator { result }
    }
}
