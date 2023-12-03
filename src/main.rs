fn main() {
    println!("Hello, world!");
    println!("{}", frame(50,30));
}

fn frame(h: i32 ,w: i32) -> String {
    
    let mut result = String::new();
    

    for frame_line in 0..h {
        for frame_col in 0..w {
            
            let mut current_char = " ";
            
            if frame_col == 0 || frame_col == w -1 {
                current_char = "║";
            }

            if frame_line == 0 {
                
                if frame_col == 0 {
                    current_char = "╔";
                } else if frame_col == w - 1 {
                    current_char = "╗";
                }else {
                    current_char = "═";
                }
            
            } else if frame_line == h - 1 {

                if frame_col == 0 {
                    current_char = "╚";
                } else if frame_col == w - 1 {
                    current_char = "╝";
                } else {
                    current_char = "═";
                }
            
            }
            result += current_char;
    }
    result += "\n"
}
return result;
}