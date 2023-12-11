fn fizz_buzz() {
    let mut result = 0;
    for idx in 1..=301 {
        match ( idx % 3 == 0, idx % 5 == 0) {
            (true, true) => {
                result += 1;
                println!("{}","fizz buzz")
            },
            (true, false) => println!("fizz"),
            (false, true) => println!("buzz"),
            (false, false) => println!("{}",idx),
        };
    }
    println!("Number of 'fizz buzz': {}", result);
}

fn main() {
    println!("Hello from main");
    fizz_buzz();
}
