use std::ops::Range;

pub const AREA_WIDTH: usize = 90;
pub const AREA_HEIGHT: usize = 50;

struct Position {
    x: usize,
    y: usize
}

struct Bounds {
    w: u16,
    h: u16,
    w_range: Range<usize>,
    h_range: Range<usize>
}

impl Bounds {
    fn contains_x(&self, x: usize) -> bool {
        self.w_range.contains(&x)
    }
    fn contains_y(&self, y: usize) -> bool {
        self.h_range.contains(&y)
    }
    fn contains(&self, pos: Position) -> bool {
        self.contains_x(pos.x) && self.contains_y(pos.y)
    }
}

pub fn new() -> PlayArea {
    return PlayArea {
        dims : Bounds{
            w: AREA_WIDTH  as u16, 
            h: AREA_HEIGHT as u16,
            w_range: (0..AREA_WIDTH),
            h_range: (0..AREA_HEIGHT)
        },
        area: vec![vec!['*'; AREA_WIDTH]; AREA_HEIGHT]
    }
}

pub struct PlayArea {
    dims : Bounds,
    area: Vec<Vec<char>>
}

impl PlayArea {
    pub fn get_width(&self) -> u16 {
        self.dims.w
    }
    pub fn get_height(&self) -> u16 {
        self.dims.h
    }
    pub fn print_to_console(&self) {
        for i in 0..self.area.len() {
            for j in 0..self.area[i].len() {
                print!("{}-", self.area[i][j])
            }
            println!();
        } 
    }
}



pub mod play_objects {
    use super::{Bounds, Position};

    trait HasPosition {
        fn fetch_position(&self) -> Position;
        fn within_bounds(&self, bounds: Bounds) -> bool {
            bounds.contains(self.fetch_position())
        }
    }

    trait HasVelocity {
        fn fetch_magnitude(&self) -> f64;
        fn fetch_direction(&self) -> f64;
        
    }
    trait CanMoveUp: HasPosition + HasVelocity {
        fn move_up(&mut self) {

        }
    }
    trait CanMoveDown: HasPosition + HasVelocity {}
    trait CanMoveLeft: HasPosition + HasVelocity {}
    trait CanMoveRight: HasPosition + HasVelocity{}

}
