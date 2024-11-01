//! Button Service Definitions

use embassy_time::{Duration, Instant, Timer};
use embedded_hal_1::digital::InputPin;
use embedded_hal_async::digital::Wait;

use crate::debounce::Debouncer;

#[derive(Debug)]
/// A struct representing a button with a generic GPIO pin.
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
    pub async fn get_press_duration(&mut self) -> Option<Duration> {
        // Wait for the button to be pressed
        if let ButtonState::ButtonPressed(_) = self.get_button_state().await {
            // Record the timestamp when the button is pressed
            let start = Instant::now();

            // Define a timeout for the button press
            let timeout = Duration::from_secs(5);

            let release_future = async {
                while let ButtonState::ButtonPressed(_) = self.get_button_state().await {
                    Timer::after(Duration::from_millis(10)).await;
                }
                Instant::now()
            };

            let end = embassy_futures::select::select(release_future, Timer::after(timeout)).await;

            match end {
                embassy_futures::select::Either::First(end) => {
                    return Some(end - start);
                }
                embassy_futures::select::Either::Second(_) => {
                    return Some(Instant::now() - start);
                }
            }
        }

        None
    }
}
