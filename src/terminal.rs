use std::os::fd::AsRawFd;

use termios::{ECHO, ICANON, TCSANOW, Termios, tcsetattr};

pub enum KeyboardAddresses {
    Status = 0xFE00,
    Data = 0xFE02,
}

/// Disables the input buffering on the terminal. This is done by grabbing
/// the current status of the terminal from termios, disabling ICANNON and ECHO.
/// This function returns the old status of the terminal
pub fn setup() -> Result<Termios, std::io::Error> {
    let stdin_fd = std::io::stdin().lock().as_raw_fd();
    let old_terminal = Termios::from_fd(stdin_fd)?;
    let mut new_terminal = old_terminal;
    new_terminal.c_lflag &= !ICANON & !ECHO;
    tcsetattr(stdin_fd, TCSANOW, &new_terminal)?;
    Ok(old_terminal)
}

/// Restores the terminal to a previous Termios status.
pub fn restore(terminal: &Termios) -> Result<(), std::io::Error> {
    let stdin_fd = std::io::stdin().lock().as_raw_fd();
    tcsetattr(stdin_fd, TCSANOW, terminal)?;
    Ok(())
}
