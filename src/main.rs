// #[derive(Debug)]
// struct Person<'a> {
//     name: &'a str,
//     age: u8,
// }
// mod foo;

fn main() {
    let name = "Orgrim Doomhammer"; // address A1
    let mut race = "Orc"; // address A2
    let number = 27; // address A3

    println!("{}\t {}\t {}", name, race, number);

    {
        let name = "Sylvanas Windrunner"; // address B1
        race = "Undead"; // address A2
        let number = 24; // address B2
        
        println!("{}\t {}\t {}", name, race, number);
    }
        
    println!("{}\t {}\t {}", name, race, number);
}
