use crate::ddd::aggregate::Aggregate;
use crate::tester::executor::AggregateTestExecutor;

/// A framework for rigorously testing the aggregate logic, one of the most important parts of
/// any DDD system.
pub struct TestFramework<A: Aggregate> {
    service: A::Service,
}

impl<A: Aggregate> TestFramework<A> {
    /// Create a test framework using the provided service.
    pub fn with(service: A::Service) -> Self {
        Self { service }
    }
}

impl<A> TestFramework<A>
where
    A: Aggregate,
{
    /// Initiates an aggregate test with no previous events.
    ///
    /// ```
    /// use base::test::framework::TestFramework;
    ///
    /// let executor = TestFramework::<MyAggregate>::with(MyService)
    ///     .given_no_previous_events();
    /// ```
    #[must_use]
    pub fn given_no_previous_events(self) -> AggregateTestExecutor<A> {
        AggregateTestExecutor {
            events: Vec::new(),
            service: self.service,
        }
    }
    /// Initiates an aggregate test with a collection of previous events.
    ///
    /// ```
    /// use base::test::framework::TestFramework;
    ///
    /// let executor = TestFramework::<MyAggregate>::with(MyService)
    ///     .given(vec![MyEvents::SomethingWasDone]);
    /// ```
    #[must_use]
    pub fn given(self, events: Vec<A::Event>) -> AggregateTestExecutor<A> {
        AggregateTestExecutor {
            events,
            service: self.service,
        }
    }
}

#[tokio::main]
pub async fn when<A: Aggregate>(
    events: Vec<A::Event>,
    command: A::Command,
    service: A::Service,
) -> Result<Vec<A::Event>, A::Error> {
    let mut aggregate = A::default();

    for event in events {
        aggregate = aggregate.apply(event);
    }

    aggregate.handle(command, &service).await
}
