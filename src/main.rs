fn main() {
    let number: i32 = 24;
    println!("value: {:?}", number);

    let pointer_number: &i32 = &number;
    println!("pointer: {:p}", pointer_number);
}
