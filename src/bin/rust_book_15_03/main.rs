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
    let csp_2 = CustomSmartPointer {
        data: String::from("my other STUFF"),
    };

    println!("CustomSmartPointer created!");
}
