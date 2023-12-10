use crate::figure::Figure;
use crate::frameblock::FrameBlock;
pub struct FrameData {
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

        FrameData { figure: (f), blocks: (frame_lines) }
    }

    pub fn get_figure(&self) -> &Figure { &self.figure }
    pub fn get_blocks(&self) -> &Vec<Vec<FrameBlock>> { &self.blocks }
}
