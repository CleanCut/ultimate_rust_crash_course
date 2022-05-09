use std::io::{Stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::style::Colored::BackgroundColor;
use crossterm::terminal::{Clear, ClearType};
use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, current_frame: &Frame, last_frame: &Frame) {
    smart_draw_frame(stdout, current_frame, last_frame);

    stdout.flush().unwrap();
}

fn smart_draw_frame(stdout: &mut Stdout, current_frame: &Frame, last_frame: &Frame) {
    for (x, col) in current_frame.iter().enumerate() {
        for (y, chr) in col.iter().enumerate() {
            if *chr != last_frame[x][y] {
                print_char_to_stdout_at_location(stdout, chr, x, y);
            }
        }
    }
}

pub fn force_render(stdout: &mut Stdout, current_frame: &Frame) {
    stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();

    draw_frame(stdout, current_frame);

    stdout.flush().unwrap();
}

fn draw_frame(stdout: &mut Stdout, current_frame: &Frame) {
    for (x, col) in current_frame.iter().enumerate() {
        for (y, chr) in col.iter().enumerate() {
            print_char_to_stdout_at_location(stdout, chr, x, y);
        }
    }
}

fn print_char_to_stdout_at_location(stdout: &mut Stdout, chr: &&str, x: usize, y: usize) {
    stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
    print!("{}", *chr);
}