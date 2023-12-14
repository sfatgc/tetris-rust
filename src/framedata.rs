use crate::figure::Figure;
use crate::frameblock::FrameBlock;
use crate::position::Position;
pub struct FrameData {
    width: usize,
    height: usize,
    figure: Figure,
    blocks: Vec<Vec<FrameBlock>>,
}

impl FrameData {
    pub fn new(frame_width: usize, frame_height: usize) -> FrameData {
        let f: Figure = Figure::new(frame_width / 2, frame_height / 8, 0);

        let mut frame_lines: Vec<Vec<FrameBlock>> = Vec::new();
        for _y in 0..frame_height {
            let mut frame_line: Vec<FrameBlock> = Vec::new();

            for _x in 0..frame_width {
                frame_line.push(FrameBlock::new())
            }

            frame_lines.push(frame_line);
        }

        FrameData {
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
        self.blocks[p.get_y()][p.get_x()].is_busy()
    }
    pub fn stats(&self) -> String {
        let b = self.figure.blocks().unwrap();
        let s = format!(
            "\n\rFrame:[{}x{}]\n\rFigure:[{}:{}, {}:{}, {}:{}, {}:{}]\n\r",
            self.get_width(),
            self.get_height(),
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
            let bb = &mut b[fb.get_y()][fb.get_x()];
            _ = bb.set_data('O');
            _ = bb.set_busy(true);
        }
    }

    pub fn move_figure_down(&mut self) -> &mut Figure {
        let _ = &self.figure.move_down();

        for fb in self.figure.blocks().unwrap() {
            if fb.get_y() >= self.get_height() - 1 || self.is_busy(fb) {
                _ = &self.figure.move_up();
                self.add_figure();
                self.figure
                    .reset(self.get_width() / 2, self.get_height() / 8, 1);
                break;
            }
        }

        &mut self.figure
    }
    pub fn move_figure_right(&mut self) -> &mut Figure {
        let _ = &self.figure.move_right();

        for fb in self.figure.blocks().unwrap() {
            if fb.get_x() >= self.get_width() - 1 || self.is_busy(fb) {
                _ = &self.figure.move_left();
                break;
            }
        }

        &mut self.figure
    }
    pub fn move_figure_left(&mut self) -> &mut Figure {
        let _ = &self.figure.move_left();

        for fb in self.figure.blocks().unwrap() {
            if fb.get_x() == 0 || self.is_busy(fb) {
                let _ = &self.figure.move_right();
                break;
            }
        }

        &mut self.figure
    }
}
