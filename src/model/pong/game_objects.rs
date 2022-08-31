pub struct Position {
    x_pos: usize,
    y_pos: usize
}

impl Position {
    pub fn new(x_pos: usize, y_pos: usize) -> Self {
        Self { x_pos, y_pos}
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Position {
    pub fn get_x_pos(&self) -> usize {self.x_pos}
    pub fn get_y_pos(&self) -> usize {self.y_pos}
    pub fn as_tuple(&self) -> (usize, usize) {(self.x_pos, self.y_pos)}
}

pub struct Dimensions {

}







