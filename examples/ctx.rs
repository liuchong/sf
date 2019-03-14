extern crate sf;

use sf::Context;

fn main() {
    let ctx0 = Context::new(1_234_567_891_011, 0).unwrap();
    let ctx1 = Context::new(1_548_067_209_841, 0).unwrap();

    loop {
        let id0 = ctx0.next_id();
        println!("id0 = {}", id0);

        let id1 = ctx1.next_id();
        println!("id1 = {}", id1);
    }
}
