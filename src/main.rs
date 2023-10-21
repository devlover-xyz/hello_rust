fn main() {
    // let mut counter = 0; // inisialisasi nilai counternya menjadi 0
    // loop {
    //     counter +=1; // tambahakan value counter dengan 1
    //     println!("The counter is now: {}", counter);
    //     if counter == 100 { // hentikan perulangan saat counter == 5
    //         break;
    //     }
    // }

    // let mut counter = 0;
    // let mut counter2 = 0;
    // println!("Now entering the first loop.");

    // 'first_loop: loop {
    //     // Berikan nama kepada loop yang pertama
    //     counter += 1;
    //     println!("The counter is now: {}", counter);

    //     if counter > 9 {
    //         // Buat loop kedua di dalam loop ini
    //         println!("Now entering the second loop.");

    //         'second_loop: loop {
    //             // sekarang kita berada pada 'second_loop
    //             println!("The second counter is now: {}", counter2);
    //             counter2 += 1;
    //             if counter2 == 3 {
    //                 break 'first_loop; // Hentikan 'first_loop sehingga kita bisa keluar dari program
    //             }
    //         }
    //     }
    // }

    // warning number karena tidak di gunakan
    // for _ in 0..3 {
    //     println!("Printing the same thing three times");
    // }

    let mut counter = 5;
    let mut modd;

    let my_number = loop {
        counter += 1;

        modd = counter % 53;
        println!("{} => {}", counter, modd);

        if counter % 53 == 3 {
            break counter;
        }
    };
    println!("MY NUMBER : {}", my_number);
}
