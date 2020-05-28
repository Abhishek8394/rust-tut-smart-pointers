// RefCells can be used to mock tests!!!
pub mod refcell_tut{
    pub trait Messenger{
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger>{
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl <'a, T> LimitTracker<'a, T> where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T>{
            LimitTracker{
                messenger,
                max,
                value: 0,
            }
        }

        /// Setvalue does not return anything.
        /// So if we want to test it, our only option is to mock the messenger and capture
        /// the messages.
        pub fn set_value(&mut self, value: usize){
            self.value = value;
            let perct_of_max = self.value as f64 / self.max as f64;
            if perct_of_max >= 1.0{
                self.messenger.send("Error: You are over the quota!");
            }
            else if perct_of_max >= 0.9 {
                self.messenger.send("Warning: You have used over 90% of your quota");
            }
            else if perct_of_max >= 0.75{
                self.messenger.send("Warning: You have used over 75% of your quota");
            }
        }
    }

}

#[cfg(test)]
mod tests{
    use super::*;
    use super::refcell_tut::*;
    use std::cell::RefCell;

    struct MockMessenger{
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger{
        fn new() -> MockMessenger {
            MockMessenger{
                sent_messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, msg: &str){
            self.sent_messages.borrow_mut().push(String::from(msg));
            // Refcell imposes at runtime that atmost 1 mutable ref and any num
            // of immutable ones by keeping counts for each type.
            // Below snippet will fail at runtime, not compile!
            // let mut m1 = self.sent_messages.borrow_mut();
            // let mut m2 = self.sent_messages.borrow_mut();

            // m1.push(String::from(msg));
            // m2.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_75_prct_warning(){
        let mck_msgr = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mck_msgr, 100);
        tracker.set_value(75);
        assert_eq!(1, mck_msgr.sent_messages.borrow().len());
        // assert_eq!("Warning: You have used over 75% of your quota", mck_msgr.sent_messages[0]);
    }
}
