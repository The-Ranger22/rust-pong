use std::ops::Range;
use game_objects::Position;


pub mod vectors;
pub mod game_objects;
pub const AREA_WIDTH: usize = 90;
pub const AREA_HEIGHT: usize = 50;



pub struct Bounds {
    w: usize,
    h: usize,
    w_range: Range<usize>,
    h_range: Range<usize>
}

impl Bounds {
    pub fn new(w: usize, h: usize) -> Self {
        Self {w, h, w_range: 0..w, h_range: 0..h}
    }
}

impl Bounds {
    fn contains_x(&self, x: usize) -> bool {
        self.w_range.contains(&x)
    }
    fn contains_y(&self, y: usize) -> bool {
        self.h_range.contains(&y)
    }
    fn contains(&self, pos: Position) -> bool {
        self.contains_x(pos.get_x_pos()) && self.contains_y(pos.get_y_pos())
    }
}

pub struct PlayArea {
    dims : Bounds,
    area: Vec<Vec<char>>
}

impl PlayArea {
    pub fn new(dims: Bounds) -> Self {
        let (w, h) = (dims.w, dims.h);
        Self {
            dims,
            area: vec![vec!['*'; w]; h]
        }
    }
}

impl Default for PlayArea {
    fn default() -> Self {
        Self::new(Bounds::new(AREA_WIDTH, AREA_HEIGHT))
    }
}


impl PlayArea {
    pub fn get_width(&self) -> usize {
        self.dims.w
    }
    pub fn get_height(&self) -> usize {
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




