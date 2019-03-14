extern crate sf;

use sf::Id;

fn main() {
    let x = 18_442_425_513_017_344;
    let y = 18_472_830_274_371_584;
    let z = 18_427_598_719_680_512;

    let id_x = Id::from_i64(x);
    let id_y = Id::from_i64(y);
    let id_z = Id::from_i64(z);

    println!("x: {}, id_x: {}", x, id_x);
    println!("y: {}, id_y: {}", y, id_y);
    println!("z: {}, id_z: {}", z, id_z);
}
