fn match_colours(rbg: (i32, i32, i32)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Each colour has at least 10"),
    }
}

fn main() {
    // menggunakan match
    let my_number: u8 = 5;
    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("It's some other number"),
    }

    // ini menggunakan if else
    let my_number: u8 = 5;
    if my_number == 0 {
        println!("it's zero");
    } else if my_number == 1 {
        println!("it's one");
    } else if my_number == 2 {
        println!("it's two");
    } else {
        println!("It's some other number");
    }

    let second_number = match my_number {
        0 => "Zero",
        5 => "Lima",
        _ => "Tidak ada",
    };

    println!("{}", second_number);

    /*
    menggunakan match guard
     */
    let children = 0;
    let married = false;

    match (children, married) {
        (children, married) if married == false => {
            println!("Not married with {} children", children)
        }
        (children, married) if children == 0 && married == true => {
            println!("Married but no children")
        }
        _ => println!("Married? {}. Number of children: {}.", married, children),
    }

    /*
    check colour
    */
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
}
