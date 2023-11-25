struct User {
    name: String,
    sign_in_count: u64,
    affliation: Vec<String>,
    active: bool,
}

fn main() {
    let user_one = User {
        name: String::from("Ajmad"),
        sign_in_count: 12,
        affliation: vec![
            String::from("Warchief of the Horde"),
            String::from("Blackrock Chieftain"),
            String::from("The Doomhammer"),
        ],
        active: false,
    };
    println!("name: {}", user_one.name);
    println!("sign-in count: {}", user_one.sign_in_count);
}
