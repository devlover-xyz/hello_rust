// #[derive(Debug)]
// struct Person<'a> {
//     name: &'a str,
//     age: u8,
// }
// mod foo;

fn main() {
    let mut message_number: i8; // address A1

    message_number = 1;
    let message1 = "hello";
    println!("message number {}: {}", message_number, message1);

    message_number = 2;
    let message2 = "world";
    println!("message number {}: {}", message_number, message2);

    let (var1, var2, mut var3): (i8, &str, char) = (24, "hello", 'A');

    println!("var1: {0}", var1); // hasilnya => var1: 24
    println!("var2: {0}", var2); // hasilnya => var2: hello
    println!("var3: {0}", var3); // hasilnya => var3: A

    var3 = 'C';
    println!("var3: {0}", var3); // hasilnya => var3: A

    let data1 = 24_i8;
    println!("data1: {0}", data1); // hasilnya => data1: 24

    let x = 5; // address C1
    println!("x: {}", x); // hasilnya => x: 5

    let x = x + 1; // address C2
    println!("x: {}", x); // hasilnya => x: 6
}
