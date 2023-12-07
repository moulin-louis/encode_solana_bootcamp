fn fizz_buzz() {
    let mut result = 0;
    for idx in 0..=301 {
        let fizz = idx % 3 == 0;
        let buzz = idx % 5 == 0;

        match (fizz, buzz) {
            (true, true) => {
                result += 1;
                println!("{}","fizz buzz")
            },
            (true, false) => println!("{}","fizz"),
            (false, true) => println!("{}","buzz"),
            (false, false) => println!("{}",idx),
        };
    }
    println!("Number of 'fizz buzz': {}", result);
}

fn main() {
    println!("Hello from main");
    fizz_buzz();
}
