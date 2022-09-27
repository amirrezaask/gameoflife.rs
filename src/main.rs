use crate::conway::{Cell};

mod conway;

fn main() {
    let cells = [
        Cell::Live,
        Cell::Dead,
        Cell::Live,

        Cell::Dead,
        Cell::Dead,
        Cell::Live,

        Cell::Live,
        Cell::Dead,
        Cell::Live,

    ];
    let mut world = conway::World::new(&cells[..], 3, 3);
    world.forward_time();
    dbg!(world);
}
