
const MEANING_OF_LIFE:u8 = 42; // no fixed memory address

static HINDSIGHT_IS:u8 = 22; // fixed address

pub fn run() {
    println!("Hello {}", MEANING_OF_LIFE);

    println!("Bye {}", HINDSIGHT_IS);
}
