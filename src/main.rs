mod endpoints;

use endpoints::by_breed;

fn main() {
    println!("{}", by_breed("hound"));
}
