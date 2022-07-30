use std::future::Future;

pub trait AsyncState {
    type Input;
    type Next: AsyncState;
    type ReturnFuture: Future<Output = Self::Next>;
    fn handle(self, input: Self::Input) -> Self::ReturnFuture;
}

pub trait State {
    type Input;
    type Next: AsyncState;
    fn handle(self, input: Self::Input) -> Self::Next;
}

#[cfg(test)]
mod fms_test {
    use crate::fsm::fms_test::async_model::AsyncVendingMachine;
    use crate::fsm::fms_test::sync_model::VendingMachine;

    #[derive(Debug, Eq, PartialEq)]
    pub enum VendingStates {
        WaitingForCoins,
        DispensingItem,
        GivingChange,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum VendingInput {
        InsertCoins(u32),
        SelectItem(String),
    }

    pub fn do_handle(current: VendingStates, input: VendingInput) -> VendingStates {
        match current {
            VendingStates::WaitingForCoins => {
                if let VendingInput::InsertCoins(coins) = input {
                    println!("Inserted {} coins", coins);

                    VendingStates::DispensingItem
                } else {
                    current
                }
            }
            VendingStates::DispensingItem => {
                if let VendingInput::SelectItem(item) = input {
                    println!("Dispensing item: {}", item);

                    VendingStates::GivingChange
                } else {
                    current
                }
            }
            VendingStates::GivingChange => {
                if let VendingInput::InsertCoins(coins) = input {
                    println!("Giving {} coins as change", coins);

                    VendingStates::WaitingForCoins
                } else {
                    current
                }
            }
        }
    }

    mod async_model {
        use std::future::Future;
        use std::pin::Pin;

        use async_trait::async_trait;

        use crate::fsm::fms_test::{do_handle, VendingInput, VendingStates};
        use crate::fsm::AsyncState;

        #[derive(Debug)]
        pub struct AsyncVendingMachine<S: AsyncState> {
            state: S,
        }

        impl AsyncVendingMachine<VendingStates> {
            pub fn new() -> Self {
                Self {
                    state: VendingStates::WaitingForCoins,
                }
            }

            pub async fn handle(self, input: VendingInput) -> AsyncVendingMachine<VendingStates> {
                AsyncVendingMachine {
                    state: self.state.handle(input).await,
                }
            }

            pub fn state(&self) -> &VendingStates {
                &self.state
            }
        }

        #[async_trait]
        impl AsyncState for VendingStates {
            type Input = VendingInput;
            type Next = VendingStates;
            type ReturnFuture = Pin<Box<dyn Future<Output = VendingStates> + Send>>;

            fn handle(self, input: Self::Input) -> Self::ReturnFuture {
                let next_state = do_handle(self, input);
                let task = async { next_state };

                Box::pin(task)
            }
        }
    }

    mod sync_model {
        use crate::fsm::fms_test::{do_handle, VendingInput, VendingStates};
        use crate::fsm::State;
        use async_trait::async_trait;

        #[async_trait]
        impl State for VendingStates {
            type Input = VendingInput;
            type Next = VendingStates;

            fn handle(self, input: Self::Input) -> Self::Next {
                do_handle(self, input)
            }
        }

        #[derive(Debug)]
        pub struct VendingMachine<S: State> {
            state: S,
        }

        impl VendingMachine<VendingStates> {
            pub fn new() -> Self {
                Self {
                    state: VendingStates::WaitingForCoins,
                }
            }

            pub fn handle(self, input: VendingInput) -> VendingMachine<VendingStates> {
                VendingMachine {
                    state: self.state.handle(input),
                }
            }

            pub fn state(&self) -> &VendingStates {
                &self.state
            }
        }
    }

    #[tokio::test]
    async fn test_async() {
        let fsm = AsyncVendingMachine::new()
            .handle(VendingInput::InsertCoins(12))
            .await
            .handle(VendingInput::SelectItem("coke".into()))
            .await;

        assert_eq!(fsm.state(), &VendingStates::GivingChange);
    }

    #[test]
    fn test_sync() {
        let fsm = VendingMachine::new()
            .handle(VendingInput::InsertCoins(12))
            .handle(VendingInput::SelectItem("coke".into()));

        assert_eq!(fsm.state(), &VendingStates::GivingChange);
    }
}
