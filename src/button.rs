//! Button Service Definitions

use embassy_time::{with_timeout, Duration, Instant, TimeoutError, Timer};
use embedded_hal_1::digital::InputPin;
use embedded_hal_async::digital::Wait;

use crate::debounce::Debouncer;

#[derive(Debug)]
/// A struct representing a button with a generic GPIO pin and a debouncer.
pub struct Button<I> {
    gpio: I,
    debouncer: Debouncer,
}

#[derive(Debug, Clone, Copy)]
/// Enum representing the state of a button.
pub enum ButtonState {
    /// The button is released and `Instant` represents the time when it was released.
    ButtonReleased(Instant),
    /// The button is pressed and `Instant` represents the time when it was pressed.
    ButtonPressed(Instant),
}

impl<I: InputPin + Wait> Button<I> {
    /// Creates a new `Button` instance with the given GPIO pin.
    pub fn new(gpio: I, debouncer: Debouncer) -> Self {
        Self { gpio, debouncer }
    }

    /// Checks button state.
    pub async fn get_button_state(&mut self) -> ButtonState {
        match self.debouncer.debounce(&mut self.gpio).await {
            true => ButtonState::ButtonPressed(Instant::now()),
            false => ButtonState::ButtonReleased(Instant::now()),
        }
    }

    /// Asynchronously gets the duration for which the button was pressed.
    pub async fn get_press_duration(&mut self, timeout: Duration) -> Option<Duration> {
        // Wait for the button to be pressed
        if let ButtonState::ButtonPressed(_) = self.get_button_state().await {
            // Record the timestamp when the button is pressed
            let start = Instant::now();

            let release_future = async {
                while let ButtonState::ButtonPressed(_) = self.get_button_state().await {
                    Timer::after(Duration::from_millis(10)).await;
                }
                Instant::now()
            };

            // Wait for the button to be released or timeout
            let end = with_timeout(timeout, release_future).await;

            return Some(match end {
                Ok(end) => end - start,
                Err(TimeoutError) => Instant::now() - start,
            });
        }

        None
    }
}
