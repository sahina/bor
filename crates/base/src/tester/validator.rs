use crate::ddd::aggregate::Aggregate;

/// Validation object for the `TestFramework` package.
pub struct AggregateResultValidator<A>
where
    A: Aggregate,
{
    pub(crate) result: Result<Vec<A::Event>, A::Error>,
}

impl<A: Aggregate> AggregateResultValidator<A> {
    /// Verifies that the expected events have been produced by the command.
    pub fn then_expect_events(self, expected_events: Vec<A::Event>) {
        let events = match self.result {
            Ok(expected_events) => expected_events,
            Err(err) => {
                panic!("expected success, received aggregate error: '{}'", err);
            }
        };
        assert_eq!(&events[..], &expected_events[..]);
    }

    /// Verifies that the result is a `UserError` and returns the internal error payload for
    /// further validation.
    pub fn then_expect_error_message(self, error_message: &str) {
        match self.result {
            Ok(events) => {
                panic!("expected error, received events: '{:?}'", events);
            }
            Err(err) => assert_eq!(err.to_string(), error_message.to_string()),
        };
    }

    /// Returns the internal error payload for validation by the user.
    pub fn inspect_result(self) -> Result<Vec<A::Event>, A::Error> {
        self.result
    }
}

impl<A> AggregateResultValidator<A>
where
    A: Aggregate,
    A::Error: PartialEq,
{
    /// Verifies that the result is the expected error.
    pub fn then_expect_error(self, expected_error: A::Error) {
        match self.result {
            Ok(events) => {
                panic!("expected error, received events: '{:?}'", events);
            }
            Err(err) => {
                assert_eq!(err, expected_error);
            }
        }
    }
}
