use std::error::Error;
use std::{io, thread};
use std::env::current_exe;
use std::sync::mpsc;
use std::time::Duration;
use soloud::*;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{frame, render};
use invaders::frame::new_frame;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // Terminal setup
    setup_terminal()?;

    play_startup_sound();

    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        render::force_render(&mut stdout, &last_frame);

        loop {
            let current_frame = match render_rx.recv() {
                Ok(frame) => frame,
                Err(_) => break,
            };

            render::render(&mut stdout, &current_frame, &last_frame);
            last_frame = current_frame;
        }
    });

    // Main game loop
    'gameloop: loop {
        let current_frame = new_frame();

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

        // below we will silently ignore errors because of:
        // let _ =
        let _ = render_tx.send(current_frame);
        thread::sleep(Duration::from_millis(1));
    }

    drop(render_tx);
    render_handle.join().unwrap();

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