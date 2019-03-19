mod hello;
use self::hello::say_hello;

fn main() {
    println!("{}", say_hello());
}
