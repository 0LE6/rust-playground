// 50: This makes programming in Rust much more convenient

// 3)
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Contents {
    path: String,
    contents: String,
}

fn get_and_display_contents(path: &str) -> Result<Contents, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut contents = String::new();

    for line in reader.lines() {
        let line = line?;
        print!("The line -> {line}\n");
        contents.push_str(&line);
        contents.push('\n');
    }
    Ok(Contents { path: path.into(), contents })
}

fn main() {
    // 1)
    // order_food("banana);
    // order_food_fast_mode("Peaches");
    
    // 2)
    // let path = String::from("secret.txt");
    // let custom_file = CustomFile { path };
    //
    // custom_file.read();
    // custom_file.mocosoft_copailot();

    let contents = get_and_display_contents("secret.txt").unwrap();
    print!("{}", contents.contents);
         
}


// 2) More examples cases
// struct CustomFile {
//     path: String
// }
//
// impl CustomFile {
//     fn read(&self) {
//         println!("Reading: '{}'", &self.path);
//     }
//
//     fn copy(&self) {
//         todo!("copy() still has to be coded!")
//     }
//
//     fn delete(&self) {
//         todo!("delete() still has to be coded!")
//     }
//
//     fn mocosoft_copailot(&self) {
//         unimplemented!("Designed for Mocosoft!")
//     }
// }


// 1) todo VS. unimplemented
// fn order_food(food: &str) {
//     todo!()
// }
//
// fn order_food_fast_mode(food: &str) {
//     unimplemented!("Potential concept!")
// }
