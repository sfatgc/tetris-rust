use tetris_rust::frameblock::FrameBlock;
use tetris_rust::framedata::FrameData;
use tetris_rust::figure::Figure;
fn main() {
    println!("Hello, world!");
    let frame_data: FrameData = FrameData::new(30, 50);
    println!("{}", render_frame(30,50, frame_data));
}

fn render_frame(w: usize, h: usize, f_d: FrameData) -> String {
    
    let f: &Figure = f_d.get_figure();
    let b: &Vec<Vec<FrameBlock>> = f_d.get_blocks();

    let mut result = String::new();
    
    (0..h).for_each(|frame_line| {
        (0..w).for_each(|frame_col| {
            eprintln!("frame_col: {}, frame_line: {}", frame_col, frame_line);
            let mut current_char = ' ';
            let current_block: &FrameBlock = &b[frame_line][frame_col];
            
            if f.is_here(frame_col, frame_line) {
                current_char = 'X';
            } else if current_block.is_busy() {
                current_char = current_block.get_data()
            }


            if frame_col == 0 || frame_col == w -1 {
                current_char = '║';
            }

            if frame_line == 0 {
                
                if frame_col == 0 {
                    current_char = '╔';
                } else if frame_col == w - 1 {
                    current_char = '╗';
                }else {
                    current_char = '═';
                }
            
            } else if frame_line == h - 1 {

                if frame_col == 0 {
                    current_char = '╚';
                } else if frame_col == w - 1 {
                    current_char = '╝';
                } else {
                    current_char = '═';
                }
            
            }
            result.push(current_char);
    });
    result += "\n"
});
return result;
}