use core::cmp::{max, min};
use core::time;
use std::time::SystemTime;

use crate::position::Position;
pub struct Figure {
    last_update: SystemTime,
    position: Position,
    figure_type: usize,
    orientation: usize,
}

impl Figure {
    pub fn new(x: usize, y: usize, t: usize, o: usize) -> Figure {
        Figure {
            position: Position::new(x, y),
            figure_type: t,
            orientation: o,
            last_update: SystemTime::now(),
        }
    }
    pub fn reset(&mut self, x: usize, y: usize, t: usize, o: usize) {
        self.position.set_x(x);
        self.position.set_y(y);
        self.figure_type = t;
        self.orientation = o;
        self.last_update = SystemTime::now();
    }
    pub fn move_up(&mut self) -> Result<&Figure, String> {
        if self.min_y() > 0 {
            self.position.set_y(self.position.get_y() - 1);
            self.last_update = SystemTime::now();
            Ok(self)
        } else {
            Err("move_up() cancelled as some of blocks are already at the top border".to_string())
        }
    }
    pub fn move_down(&mut self) -> Result<&Figure, String> {
        self.position.set_y(self.position.get_y() + 1);
        self.last_update = SystemTime::now();
        Ok(self)
    }
    pub fn move_left(&mut self) -> Result<&Figure, String> {
        if self.min_x() > 0 {
            self.position.set_x(self.position.get_x() - 1);
            self.last_update = SystemTime::now();
            Ok(self)
        } else {
            Err("move_left() cancelled as some of blocks are already at left border".to_string())
        }
    }
    pub fn move_right(&mut self) -> Result<&Figure, String> {
        self.position.set_x(self.position.get_x() + 1);
        self.last_update = SystemTime::now();
        Ok(self)
    }
    pub fn next_form(&mut self) -> Result<&Figure, String> {
        self.orientation += 1;
        self.last_update = SystemTime::now();
        Ok(self)
    }
    pub fn prev_form(&mut self) -> Result<&Figure, String> {
        self.orientation -= 1;
        self.last_update = SystemTime::now();
        Ok(self)
    }

    pub fn max_types() -> usize {
        3
    }
    pub fn max_orientations() -> usize {
        8
    }
    pub fn updated(&self) -> time::Duration {
        self.last_update.elapsed().unwrap()
    }

    pub fn min_x(&self) -> usize {
        let b = self.blocks().unwrap();
        min(
            min(b[0].get_x(), b[1].get_x()),
            min(b[2].get_x(), b[3].get_x()),
        )
    }
    pub fn min_y(&self) -> usize {
        let b = self.blocks().unwrap();
        min(
            min(b[0].get_y(), b[1].get_y()),
            min(b[2].get_y(), b[3].get_y()),
        )
    }
    pub fn max_x(&self) -> usize {
        let b = self.blocks().unwrap();
        max(
            max(b[0].get_x(), b[1].get_x()),
            max(b[2].get_x(), b[3].get_x()),
        )
    }
    pub fn max_y(&self) -> usize {
        let b = self.blocks().unwrap();
        max(
            max(b[0].get_y(), b[1].get_y()),
            max(b[2].get_y(), b[3].get_y()),
        )
    }
    pub fn is_here(&self, a_x: usize, a_y: usize) -> bool {
        let b = self.blocks().unwrap();

        b[0].is_here(a_x, a_y)
            || b[1].is_here(a_x, a_y)
            || b[2].is_here(a_x, a_y)
            || b[3].is_here(a_x, a_y)
    }

    pub fn blocks(&self) -> Result<[Position; 4], String> {
        match self.figure_type % Figure::max_types() {
            0 => Ok(self.blocks_line()),
            1 => Ok(self.blocks_square()),
            2 => Ok(self.blocks_l()),
            _ => Err("Figure::figure_type % 3 is not 0..2".to_string()),
        }
    }

    fn blocks_l(&self) -> [Position; 4] {
        let p_x: usize = self.position.get_x();
        let p_y: usize = self.position.get_y();
        match self.orientation % 8 {
            0 => [
                Position::new(p_x, p_y),
                Position::new(p_x, p_y + 1),
                Position::new(p_x, p_y + 2),
                Position::new(p_x + 1, p_y + 2),
            ],
            1 => [
                Position::new(p_x, p_y),
                Position::new(p_x + 1, p_y),
                Position::new(p_x + 2, p_y),
                Position::new(p_x + 2, p_y - 1),
            ],
            2 => [
                Position::new(p_x, p_y),
                Position::new(p_x, p_y - 1),
                Position::new(p_x, p_y - 2),
                Position::new(p_x - 1, p_y - 2),
            ],
            3 => [
                Position::new(p_x, p_y),
                Position::new(p_x - 1, p_y),
                Position::new(p_x - 2, p_y),
                Position::new(p_x - 2, p_y + 1),
            ],
            4 => [
                Position::new(p_x, p_y),
                Position::new(p_x, p_y + 1),
                Position::new(p_x, p_y + 2),
                Position::new(p_x - 1, p_y + 2),
            ],
            5 => [
                Position::new(p_x, p_y),
                Position::new(p_x + 1, p_y),
                Position::new(p_x + 2, p_y),
                Position::new(p_x + 2, p_y + 1),
            ],
            6 => [
                Position::new(p_x, p_y),
                Position::new(p_x, p_y - 1),
                Position::new(p_x, p_y - 2),
                Position::new(p_x + 1, p_y - 2),
            ],
            _ => [
                Position::new(p_x, p_y),
                Position::new(p_x - 1, p_y),
                Position::new(p_x - 2, p_y),
                Position::new(p_x - 2, p_y - 1),
            ],
        }
    }

    fn blocks_line(&self) -> [Position; 4] {
        let p_x: usize = self.position.get_x();
        let p_y: usize = self.position.get_y();
        match self.orientation % 4 {
            0 => [
                Position::new(p_x, p_y),
                Position::new(p_x + 1, p_y),
                Position::new(p_x + 2, p_y),
                Position::new(p_x + 3, p_y),
            ],
            1 => [
                Position::new(p_x, p_y),
                Position::new(p_x, p_y - 1),
                Position::new(p_x, p_y - 2),
                Position::new(p_x, p_y - 3),
            ],
            2 => [
                Position::new(p_x, p_y),
                Position::new(p_x - 1, p_y),
                Position::new(p_x - 2, p_y),
                Position::new(p_x - 3, p_y),
            ],
            _ => [
                Position::new(p_x, p_y),
                Position::new(p_x, p_y + 1),
                Position::new(p_x, p_y + 2),
                Position::new(p_x, p_y + 3),
            ],
        }
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
