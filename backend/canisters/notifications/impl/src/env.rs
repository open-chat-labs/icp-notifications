use candid::Principal;
use types::TimestampMillis;

pub trait Environment {
    fn now(&self) -> TimestampMillis;
    fn caller(&self) -> Principal;
}

pub struct CanisterEnvironment {}

impl Environment for CanisterEnvironment {
    fn now(&self) -> u64 {
        ic_cdk::api::time()
    }

    fn caller(&self) -> Principal {
        ic_cdk::caller()
    }
}

pub struct EmptyEnvironment {}

impl Environment for EmptyEnvironment {
    fn now(&self) -> u64 {
        unimplemented!()
    }

    fn caller(&self) -> Principal {
        unimplemented!()
    }
}

#[cfg(test)]
pub struct TestEnvironment {
    pub now: u64,
    pub caller: Principal,
}

#[cfg(test)]
impl Environment for TestEnvironment {
    fn now(&self) -> u64 {
        self.now
    }

    fn caller(&self) -> Principal {
        self.caller
    }
}
