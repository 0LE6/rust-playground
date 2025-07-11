// 34: Cool move syntax in Rust

fn main() {
   let user = User {
        id: 0,
        username: String::from("Rusty"),
        email: String::from("rusty@rs.com"),
    };

    let updated_user = User {
        email: String::from("new_rusty@rs.com"),
        ..user
    };

    println!("Updated user email -> {}", updated_user.email);

    // esto no se podría hacero (se movió el campo)
    // println!("Updated user email -> {}", user.username); 
    
    // pero el mail del user sigue siendo accesible, porque 
    // el updated_user lo movió, no lo sustituyó 

    println!("Updated user email -> {}", user.email);

    // Rusty case:
    //  -   username es de tipo String, que no implementa el trait Copy.
    //  -   id es de tipo i32, que sí implementa el trait Copy.

    //  Cuando hacemos ..user en la construcción de updated_user, 
    //  Rust mueve los campos que no implementan Copy (como String), 
    //  porque mover un String implica transferir la propiedad de su heap.

    //  Pero para los tipos que implementan Copy 
    //  (como i32, bool, tipos primitivos en general), 
    //  Rust hace una copia bit a bit automática en vez de mover.
    println!("Updated user email -> {}", user.id);
}

struct User {
    id: i32,
    username: String, 
    email: String,
}

// struct Fruit {
//     name: String,
//     grams: i32,
//     oruice: f32,
// }
