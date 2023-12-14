use crate::position::Position;
pub struct Figure {
    position: Position,
    figure_type: i32,
}

impl Figure {
    pub fn new(x: usize, y: usize, t: i32) -> Figure {
        Figure {
            position: Position::new(x, y),
            figure_type: t,
        }
    }
    pub fn reset(&mut self, x: usize, y: usize, t: i32) {
        self.position.set_x(x);
        self.position.set_y(y);
        self.figure_type = t;
    }
    pub fn move_up(&mut self) -> Result<&Figure, String> {
        self.position.set_y(self.position.get_y() - 1);
        Ok(self)
    }
    pub fn move_down(&mut self) -> Result<&Figure, String> {
        self.position.set_y(self.position.get_y() + 1);
        Ok(self)
    }
    pub fn move_left(&mut self) -> Result<&Figure, String> {
        self.position.set_x(self.position.get_x() - 1);
        Ok(self)
    }
    pub fn move_right(&mut self) -> Result<&Figure, String> {
        self.position.set_x(self.position.get_x() + 1);
        Ok(self)
    }

    pub fn is_here(&self, a_x: usize, a_y: usize) -> bool {
        let b = self.blocks().unwrap();

        b[0].is_here(a_x, a_y)
            || b[1].is_here(a_x, a_y)
            || b[2].is_here(a_x, a_y)
            || b[3].is_here(a_x, a_y)
    }

    pub fn blocks(&self) -> Result<[Position; 4], String> {
        match self.figure_type % 2 {
            0 => Ok(self.blocks_line()),
            1 => Ok(self.blocks_square()),
            _ => Err("Figure::figure_type % 2 is not 0 or 1".to_string()),
        }
    }

    fn blocks_line(&self) -> [Position; 4] {
        let p_x: usize = self.position.get_x();
        let p_y: usize = self.position.get_y();
        [
            Position::new(p_x, p_y),
            Position::new(p_x + 1, p_y),
            Position::new(p_x + 2, p_y),
            Position::new(p_x + 3, p_y),
        ]
    }

    fn blocks_square(&self) -> [Position; 4] {
        let p_x: usize = self.position.get_x();
        let p_y: usize = self.position.get_y();
        [
            Position::new(p_x, p_y),
            Position::new(p_x + 1, p_y),
            Position::new(p_x, p_y + 1),
            Position::new(p_x + 1, p_y + 1),
        ]
    }
}
