//! Button Interpreter Module

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
pub async fn check_button_press<I: InputPin + Wait>(button: &mut Button<I>) -> Message {
    let duration = button.get_press_duration().await;
    match duration {
        Some(duration) => match duration.as_millis() {
            0..=2000 => Message::ShortPress,
            2001..=5000 => Message::LongPress,
            5001.. => Message::PressAndHold,
        },
        _ => Message::Ignore,
    }
}
