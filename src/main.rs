fn main() {
    let mut counter = 0; // inisialisasi nilai counternya menjadi 0
    loop {
        counter +=1; // tambahakan value counter dengan 1
        println!("The counter is now: {}", counter);
        if counter == 100 { // hentikan perulangan saat counter == 5
            break;
        }
    }
}
