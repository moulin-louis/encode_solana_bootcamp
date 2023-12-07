fn fizz_buzz() {
    for idx in 0..=301 {
        if idx % 3  == 0{
            print!("fizz");
            if idx % 5 == 0 {
                print!(" buzz");
            }
        }
        if idx % 5 == 0 {
            print!("buzz");
        }
        
    }
}

fn main() {
    println!("Hello from main");
    fizz_buzz();
}
