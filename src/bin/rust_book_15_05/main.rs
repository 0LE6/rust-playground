// RefCell<T> and the Interior Mutability Pattern

/*
* Rc<T> enables multiple owners of 
the same data; Box<T> and RefCell<T> 
have single owners.

* Box<T> allows immutable or mutable 
borrows checked at compile time; Rc<T> 
allows only immutable borrows checked 
at compile time; RefCell<T> allows 
immutable or mutable borrows checked at runtime.

* Because RefCell<T> allows mutable 
borrows checked at runtime, you can 
mutate the value inside the RefCell<T> 
even when the RefCell<T> is immutable.
*/

use std::{cell::RefCell, rc::Rc};
use crate::List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(
        Cons(Rc::clone(&value), Rc::new(Nil))
    );

    let b = Cons(
        Rc::new(RefCell::new(3)), Rc::clone(&a)
    );

    let c = Cons(
        Rc::new(RefCell::new(4)), Rc::clone(&a)
    );

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    // lists b and c can both refer to a

}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T>
where  
    T: Messenger,
{
   pub fn new(
       messenger: &'a T, 
       max: usize
    ) -> LimitTracker<'a, T> {
       LimitTracker { 
           messenger,
           value: 0,
           max,
       }
   }

   pub fn set_value(
       &mut self,
       value: usize,
   ) {
        self.value = value;

        let percentage_of_max =
            self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send(
                "Error: You're over your quota!'"
                );
        } else if percentage_of_max >= 0.9 {
            self.messenger.send(
                "Urgent warning: U've used up over 90% quota!"
            );
        } else if percentage_of_max >= 0.75 {
            self.messenger.send(
                "Warning: You've used up over 75% of quota!'"
            );
        }
   }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Correct form
            // self.sent_messages
            //     .borrow_mut()
            //     .push(String::from(message));
            
            // deliberately trying to create two
            // mutable borrows active for the same 
            // scope to illustrate that RefCell<T> 
            // prevents us from doing this at runtime.
            let mut one_borrow = 
                self.sent_messages.borrow_mut();
            let mut two_borrow = 
                self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker =
            LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(
            mock_messenger
                .sent_messages
                .borrow()
                .len(), 
            1
        );

    }
}
