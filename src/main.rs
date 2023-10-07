fn main() {
    let numerik1 = 24;
    let numerik2: i8 = 20;
    let numerik3: i16 = 32767;

    println!("{} | {} | {}", numerik1, numerik2, numerik3);

    let fp1: f32 = 3.14;
    let fp2: f32 = 3.1415926535;

    println!("{} | {:.4}", fp1, fp2);
    // output ==> 3.14 | 3.14159

    let var5 = r#"
        {
            "name": "tim drake",
            "gender": "male"
        }
    "#;
    println!("{}", var5);

    const LABEL: &str = "nilai pi adalah:";
    const PI: f32 = 22.0 / 7.0;

    println!("{} {}", LABEL, PI);
}
