use candid::Principal;
use types::TimestampMillis;

pub trait Environment {
    fn now(&self) -> TimestampMillis;
    fn caller(&self) -> Principal;
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
