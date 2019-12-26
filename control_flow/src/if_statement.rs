
pub fn check_temp() {
    let temp = 9;

    if temp > 30 {
        println!("It is above 30.");
    } else if temp < 10 {
        println!("It is below 10");
    } else {
        println!("It is between 10 and 30");
    }

    let weather = if temp > 20 {
        "Good"
    } else {
        "Not so good"
    };

    println!("{} weather!", weather);

}