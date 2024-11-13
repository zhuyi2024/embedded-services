//! Button Interpreter Module

use embassy_time::Duration;
use embedded_hal_1::digital::InputPin;
use embedded_hal_async::digital::Wait;

use crate::button::Button;

#[derive(Clone, Copy)]
/// Enum representing the different types of messages that can be sent by the button.
pub enum Message {
    /// Button pressed for long duration.
    LongPress,
    /// Button pressed for short duration.
    ShortPress,
    /// Button pressed and held.
    PressAndHold,
    /// Ignore the button press.
    Ignore,
}

/// Checks the button press duration and returns the corresponding state.
pub async fn check_button_press<I: InputPin + Wait>(button: &mut Button<I>, timeout: Duration) -> Message {
    if let Some(duration) = button.get_press_duration(timeout).await {
        // Handle timeout case
        if duration.as_millis() >= timeout.as_millis() {
            return Message::PressAndHold;
        }

        // Handle other button press durations
        match duration.as_millis() {
            0..=1999 => Message::ShortPress,
            2000.. => Message::LongPress,
        }
    } else {
        // Ignore button release which timed out
        Message::Ignore
    }
}
