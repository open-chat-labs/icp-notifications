use candid::Principal;
use types::TimestampMillis;

pub trait Environment {
    fn now(&self) -> TimestampMillis;
    fn caller(&self) -> Principal;
}
