use std::sync::Mutex;
use num::Integer;



pub struct AutoincrementCounter<T> {
    value: Mutex<T>,
}

impl<T: Integer + Copy> AutoincrementCounter<T> {
    
    pub fn post_increment(&self) -> T {
        let mut lock = self.value.lock().expect("Cannot lock a poisoned mutex");

        let value : T = *lock;
        lock.inc();

        value
    }
    
    pub fn pre_increment(&self) -> T {
        let mut lock = self.value.lock().expect("Cannot lock a poisoned mutex");
        lock.inc();

        *lock
    }

}

impl<T> From<T> for AutoincrementCounter<T> {

    fn from(value: T) -> Self {
        Self{value: value.into()}
    }

}
