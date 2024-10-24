//! Power Button Service Definitions

use embassy_time::Instant;
use embedded_hal_1::digital::InputPin;
use embedded_hal_async::digital::Wait;

#[derive(Debug, Clone, Copy)]
/// Enum representing the state of a button.
pub enum State {
    /// The button is not pressed.
    NotPressed,
    /// The button is pressed and the `Duration` represents the time it has been pressed for.
    Pressed(embassy_time::Duration),
}

#[derive(Debug)]
/// A struct representing a button with a generic GPIO pin.
pub struct Button<I> {
    gpio: I,
}

impl<I: InputPin + Wait> Button<I> {
    /// Creates a new `Button` instance with the given GPIO pin.
    pub fn new(gpio: I) -> Self {
        Self { gpio }
    }

    /// Asynchronously gets the current state of the button.
    ///
    /// This method waits for the button to be pressed and then released,
    /// and returns the state of the button along with the duration it was pressed for.
    pub async fn get_state(&mut self) -> State {
        if let Err(_) = self.gpio.wait_for_low().await {
            // Handle the error by returning NotPressed
            return State::NotPressed;
        }
        let start = Instant::now();
        if let Err(_) = self.gpio.wait_for_high().await {
            // Handle the error by returning NotPressed
            return State::NotPressed;
        }
        let end = Instant::now();

        let press_duration = end - start;

        if press_duration.as_millis() == 0 {
            State::NotPressed
        } else {
            State::Pressed(press_duration)
        }
    }
}
