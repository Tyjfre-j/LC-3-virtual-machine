use std::{
    io,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal,
};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

/// RAII guard: restores console settings when dropped.
pub struct InputBufferingGuard;

impl Drop for InputBufferingGuard {
    fn drop(&mut self) {
        restore_input_buffering();
    }
}

/// Enable raw input so key presses are delivered immediately (no Enter).
pub fn disable_input_buffering() -> io::Result<InputBufferingGuard> {
    terminal::enable_raw_mode()
        .map(|_| InputBufferingGuard)
        .map_err(|e| io::Error::other(e.to_string()))
}

pub fn restore_input_buffering() {
    let _ = terminal::disable_raw_mode();
}

/// Install a Ctrl+C handler that restores console mode and requests the VM to stop.
pub fn install_ctrlc_handler() -> io::Result<()> {
    ctrlc::set_handler(|| {
        INTERRUPTED.store(true, Ordering::SeqCst);
        restore_input_buffering();
        let _ = std::io::Write::write_all(&mut std::io::stdout(), b"\n");
    })
    .map_err(|e| io::Error::other(e.to_string()))
}

pub fn interrupted() -> bool {
    INTERRUPTED.load(Ordering::Relaxed)
}

pub(crate) fn check_key() -> bool {
    event::poll(Duration::from_millis(0)).unwrap_or(false)
}

pub(crate) fn get_char() -> u8 {
    loop {
        if interrupted() {
            return 0;
        }

        if !event::poll(Duration::from_millis(10)).unwrap_or(false) {
            continue;
        }

        match event::read() {
            Ok(Event::Key(key)) => {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    INTERRUPTED.store(true, Ordering::SeqCst);
                    restore_input_buffering();
                    return 0;
                }

                if let KeyCode::Char(ch) = key.code {
                    return ch as u8;
                }
            }
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
