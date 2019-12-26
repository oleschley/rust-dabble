
pub fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        
        x *= 2;

        if x == 64 { continue; }

        println!{"{}", x};

    }
}