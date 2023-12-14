pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_y(&self) -> usize {
        self.y
    }
    pub fn set_x(&mut self, a_x: usize) -> usize {
        self.x = a_x;
        self.x
    }
    pub fn set_y(&mut self, a_y: usize) -> usize {
        self.y = a_y;
        self.y
    }
    pub fn is_here(&self, a_x: usize, a_y: usize) -> bool {
        self.x == a_x && self.y == a_y
    }
}
