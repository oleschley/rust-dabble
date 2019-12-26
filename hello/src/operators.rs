pub fn run() {
    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a += 1;
    println!("{}", a);

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed is {}", b, b_cubed);
    println!("{} to the power of Pi is {}", b, b_to_pi);
}