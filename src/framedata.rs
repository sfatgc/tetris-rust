use crate::figure::Figure;
use crate::frameblock::FrameBlock;
pub struct FrameData {
    width: usize,
    height: usize,
    figure: Figure,
    blocks: Vec<Vec<FrameBlock>>
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

        FrameData { width: (frame_width), height: (frame_height), figure: (f), blocks: (frame_lines) }
    }

    pub fn get_figure(&self) -> &Figure { &self.figure }
    pub fn get_blocks(&self) -> &Vec<Vec<FrameBlock>> { &self.blocks }
    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }
    pub fn move_figure_down(&mut self) -> &mut Figure { let _ = &self.figure.move_down(); &mut self.figure }
    pub fn move_figure_right(&mut self) -> &mut Figure { let _ = &self.figure.move_right(); &mut self.figure }
    pub fn move_figure_left(&mut self) -> &mut Figure { let _ = &self.figure.move_left(); &mut self.figure }
}
