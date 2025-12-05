// Running Code on Cleanup with the Drop Trait

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Droppping CustomSmartPointer with data {}!",
            self.data
        );
    }
}

fn main() {
    let csp = CustomSmartPointer {
        data: String::from("my STUFF"),
    };
    // let csp_2 = CustomSmartPointer {
    //     data: String::from("other STUFF"),
    // };

    println!("CustomSmartPointer created!");
    // csp.drop();
    // consider using `drop` function
    // |
    // 25 -     csp.drop();
    // 25 +     drop(csp);
    drop(csp);
    println!("CustomSmartPointer created!");

    // The text Dropping CustomSmartPointer with data 
    // `my STUFF` is printed between the CustomSmartPointer 
    // created. and CustomSmartPointer dropped before the 
    // end of main. text, showing that the drop method 
    // code is called to drop c at that point.
}
