// RefCells can be used to mock tests!!!
pub mod refcell_tut{
    pub trait Messenger{
        fn send(&mut self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger>{
        messenger: &'a mut T,
        value: usize,
        max: usize,
    }

    impl <'a, T> LimitTracker<'a, T> where
        T: Messenger,
    {
        pub fn new(messenger: &mut T, max: usize) -> LimitTracker<T>{
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
    // use super::*;
    use super::refcell_tut::*;

    struct MockMessenger{
        sent_messages: Vec<String>,
    }

    impl MockMessenger{
        fn new() -> MockMessenger {
            MockMessenger{
                sent_messages: Vec::new(),
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&mut self, msg: &str){
            self.sent_messages.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_75_prct_warning(){
        let mut mck_msgr = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mut mck_msgr, 100);
        tracker.set_value(75);
        assert_eq!(1, mck_msgr.sent_messages.len());
        // assert_eq!("Warning: You have used over 75% of your quota", mck_msgr.sent_messages[0]);
    }
}
