// All the Places Patterns Can Be Used

fn main() {
    let fav_color: Option<&str> = Some("banana");

    let is_tuesday: bool = false;

    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fav_color {
        println!(
            "Using your fav color, {color}, as bg"
        );
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as bg color");
        } else {
            println!("Using orange as bg color");
        }
    } else {
        println!("Using blue as bg color");
    }
}
