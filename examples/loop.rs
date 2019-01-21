extern crate sf;

use sf::Generator;

fn main() {
    let gen = Generator::new(1_548_067_209_841, 0).unwrap();

    loop {
        let id = gen.next_id().unwrap();

        println!("id = {}", id);
        println!("id = {:x}", id);
        println!("id = {:X}", id);
    }
}
