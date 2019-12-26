
pub fn run() {
    let a = 132;
    
    {
        let b = 456;
        let a = 777;
        println!("a = {}", a);
        println!("b is {}", b);
    }

    println!("a = {}", a);
}