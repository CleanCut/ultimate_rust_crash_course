use std::error::Error;
use std::io;
use std::time::Duration;
use soloud::*;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // Terminal setup
    setup_terminal()?;

    play_startup_sound();

    // Main game loop
    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }

    // Terminal cleanup
    cleanup_terminal()?;

    Ok(())
}

fn setup_terminal() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    Ok(())
}

fn cleanup_terminal() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn play_startup_sound() {
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load_mem(include_bytes!("../startup.wav")).unwrap();
    sl.play(&wav);

    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}