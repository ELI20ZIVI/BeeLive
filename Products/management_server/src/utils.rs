use std::sync::Mutex;
use std::ops::AddAssign;
use num::Integer;



struct AutoincrementCounter<T: num::Integer + std::ops::AddAssign<T>> {
    value: Mutex<T>,
}

impl<T: Integer + AddAssign<T> + Copy> AutoincrementCounter<T> {
    
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
