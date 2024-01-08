use crate::constants::{H_INDENT, W_INDENT};
use crate::figure::Figure;
use crate::frameblock::FrameBlock;
use crate::position::Position;
use rand::Rng;
pub struct FrameData {
    next_figure_type: usize,
    next_figure_orienttion: usize,
    round: usize,
    score: usize,
    width: usize,
    height: usize,
    figure: Figure,
    blocks: Vec<Vec<FrameBlock>>,
}

impl FrameData {
    pub fn new(frame_width: usize, frame_height: usize) -> FrameData {
        let mut rng = rand::thread_rng();

        let f: Figure = Figure::new(
            W_INDENT + frame_width / 2,
            H_INDENT + frame_height / 8,
            rng.gen_range(0..Figure::max_types()),
            rng.gen_range(0..Figure::max_orientations()),
        );

        let mut frame_lines: Vec<Vec<FrameBlock>> = Vec::new();
        for _y in 0..frame_height {
            let mut frame_line: Vec<FrameBlock> = Vec::new();

            for _x in 0..frame_width {
                frame_line.push(FrameBlock::new())
            }

            frame_lines.push(frame_line);
        }

        FrameData {
            next_figure_type: (rng.gen_range(0..Figure::max_types())),
            next_figure_orienttion: (rng.gen_range(0..Figure::max_orientations())),
            round: (0),
            score: (0),
            width: (frame_width),
            height: (frame_height),
            figure: (f),
            blocks: (frame_lines),
        }
    }

    pub fn get_figure(&self) -> &Figure {
        &self.figure
    }
    pub fn get_blocks(&self) -> &Vec<Vec<FrameBlock>> {
        &self.blocks
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn is_busy(&self, p: Position) -> bool {
        if p.get_y() >= H_INDENT && p.get_x() >= W_INDENT {
            self.blocks[p.get_y() - H_INDENT][p.get_x() - W_INDENT].is_busy()
        } else {
            false
        }
    }
    fn figure_cannot_be_here(&self, f: &Figure) -> Result<bool, String> {
        for fb in f.blocks().unwrap() {
            if fb.get_y() >= self.get_height() + H_INDENT - 1 {
                return Err(format!(
                    "fb.get_y()=={} >= {}==self.get_height() + H_INDENT - 1",
                    fb.get_y(),
                    self.get_height() + H_INDENT - 1
                ));
            }
            /* fb.get_y() <= H_INDENT */
            if fb.get_x() <= W_INDENT {
                return Err(format!(
                    "fb.get_x()=={} <= {}==W_INDENT",
                    fb.get_x(),
                    W_INDENT
                ));
            }
            if fb.get_x() >= self.get_width() + W_INDENT - 1 {
                return Err(format!(
                    "fb.get_x()=={} >= {}==self.get_width() + W_INDENT - 1",
                    fb.get_x(),
                    self.get_width() + W_INDENT - 1
                ));
            }
            if self.is_busy(fb) {
                return Err(format!("self.is_busy(fb): true"));
            }
        }
        Ok(true)
    }

    pub fn stats(&self) -> String {
        let b = self.figure.blocks().unwrap();
        let s = format!(
            "\n\rFrame:[{}x{}]; Round: {}; Score: {}\n\rFigure:[{}:{}, {}:{}, {}:{}, {}:{}]\n\r",
            self.get_width(),
            self.get_height(),
            self.round,
            self.score,
            b[0].get_x(),
            b[0].get_y(),
            b[1].get_x(),
            b[1].get_y(),
            b[2].get_x(),
            b[2].get_y(),
            b[3].get_x(),
            b[3].get_y()
        );
        s
    }

    pub fn add_figure(&mut self) {
        let b: &mut Vec<Vec<FrameBlock>> = &mut self.blocks;
        for fb in self.figure.blocks().unwrap() {
            if fb.get_y() >= H_INDENT && fb.get_x() >= W_INDENT {
                let bb = &mut b[fb.get_y() - H_INDENT][fb.get_x() - W_INDENT];
                _ = bb.set_data('◘');
                _ = bb.set_busy(true);
            }
        }
        self.round += 1;
    }

    pub fn turn_figure(&mut self) -> &mut Figure {
        _ = self.figure.next_form();
        for fb in self.figure.blocks().unwrap() {
            if fb.get_y() <= H_INDENT
                || fb.get_y() >= self.get_height() + H_INDENT - 1
                || fb.get_x() <= W_INDENT
                || fb.get_x() >= self.get_width() + W_INDENT - 1
                || self.is_busy(fb)
            {
                _ = self.figure.prev_form();
                break;
            }
        }

        &mut self.figure
    }
    pub fn move_figure_down(&mut self) -> Result<&mut Figure, String> {
        let _ = &self.figure.move_down();

        match self.figure_cannot_be_here(&self.figure) {
            Err(_) => {
                _ = &self.figure.move_up();
                self.add_figure();
                self.figure.reset(
                    W_INDENT + self.get_width() / 2,
                    H_INDENT + self.get_height() / 8,
                    self.next_figure_type,
                    self.next_figure_orienttion,
                );
                let mut rng = rand::thread_rng();
                self.next_figure_type = rng.gen_range(0..Figure::max_types());
                self.next_figure_orienttion = rng.gen_range(0..Figure::max_orientations());

                match self.figure_cannot_be_here(&self.figure) {
                    Err(t) => Err(t),
                    Ok(_) => Ok(&mut self.figure),
                }
            }
            Ok(_) => Ok(&mut self.figure),
        }
    }
    pub fn move_figure_right(&mut self) -> &mut Figure {
        let _ = &self.figure.move_right();

        for fb in self.figure.blocks().unwrap() {
            if fb.get_x() >= self.get_width() + W_INDENT - 1 || self.is_busy(fb) {
                _ = &self.figure.move_left();
                break;
            }
        }

        &mut self.figure
    }
    pub fn move_figure_left(&mut self) -> &mut Figure {
        match &self.figure.move_left() {
            Ok(_) => {
                for fb in self.figure.blocks().unwrap() {
                    if fb.get_x() <= W_INDENT || self.is_busy(fb) {
                        let _ = &self.figure.move_right();
                        break;
                    }
                }
            }
            Err(_) => {}
        }

        &mut self.figure
    }
}
