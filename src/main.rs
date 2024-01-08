extern crate termion;
use std::io::{stdout, Write};

use std::thread::sleep;
use std::time::Duration;
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use tetris_rust::constants::{H_INDENT, W_INDENT};
use tetris_rust::figure::Figure;
use tetris_rust::frameblock::FrameBlock;
use tetris_rust::framedata::FrameData;

fn main() {
    let mut frame_data: FrameData = FrameData::new(15, 20);

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    println!("Hello, world!");

    let mut stdin = async_stdin().keys();
    //let stdin = stdin();

    loop {
        let mut command: String = "".to_string();

        if frame_data.get_figure().updated().as_millis() >= 500 {
            match frame_data.move_figure_down() {
                Ok(_) => {}
                Err(t) => {
                    println!("GAME OVER: {}", t);
                    break;
                }
            }
        }

        let c = stdin.next();

        match c {
            Some(input_key) => match input_key.unwrap() {
                Key::Char('q') => break,
                Key::Char(c) => command = format!("{}", c),
                Key::Alt(c) => command = format!("^{}", c),
                Key::Ctrl(c) => command = format!("*{}", c),
                Key::Esc => command = format!("ESC"),
                Key::Left => {
                    frame_data.move_figure_left();
                    command = format!("←")
                }
                Key::Right => {
                    frame_data.move_figure_right();
                    command = format!("→")
                }
                Key::Up => {
                    frame_data.turn_figure();
                    command = format!("↑")
                }
                Key::Down => {
                    command = format!("↓");
                    match frame_data.move_figure_down() {
                        Ok(_) => {}
                        Err(t) => {
                            println!("GAME OVER: {}", t);
                            break;
                        }
                    }
                }
                Key::Backspace => command = format!("×"),
                _ => {}
            },
            _ => {}
        }

        write!(
            stdout,
            "{}{}{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
            render_frame(&mut frame_data),
            command,
            frame_data.stats()
        )
        .unwrap();

        stdout.flush().unwrap();
        sleep(Duration::from_millis(100));
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn render_frame(f_d: &FrameData) -> String {
    let w: usize = f_d.get_width() + W_INDENT;
    let h: usize = f_d.get_height() + H_INDENT;
    let f: &Figure = f_d.get_figure();
    let b: &Vec<Vec<FrameBlock>> = f_d.get_blocks();

    let mut result = String::new();

    (0..h).for_each(|frame_line| {
        (0..w).for_each(|frame_col| {
            /* eprintln!("frame_col: {}, frame_line: {}", frame_col, frame_line); */
            let mut current_char = ' ';

            if frame_col >= W_INDENT && frame_line >= H_INDENT {
                let current_block: &FrameBlock = &b[frame_line - H_INDENT][frame_col - W_INDENT];

                if f.is_here(frame_col, frame_line) {
                    current_char = '█';
                } else if current_block.is_busy() {
                    current_char = current_block.get_data()
                }
            }

            if (frame_line > H_INDENT && frame_line < h)
                && (frame_col == W_INDENT || frame_col == w - 1)
            {
                current_char = '║';
            }

            if frame_line == H_INDENT {
                if frame_col == W_INDENT {
                    current_char = '╔';
                } else if frame_col == w - 1 {
                    current_char = '╗';
                } else if frame_col > W_INDENT && frame_col < w {
                    current_char = '═';
                }
            } else if frame_line == h - 1 {
                if frame_col == W_INDENT {
                    current_char = '╚';
                } else if frame_col == w - 1 {
                    current_char = '╝';
                } else if frame_col > W_INDENT && frame_col < w {
                    current_char = '═';
                }
            }
            result.push(current_char);
            if frame_col > W_INDENT && frame_col < w - 1 {
                result.push(current_char);
            }
        });
        result += "\n\r"
    });
    return result;
}
