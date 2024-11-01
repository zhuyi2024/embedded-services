//! Debounce Module

use embassy_time::{Duration, Timer};
use embedded_hal_1::digital::InputPin;

#[derive(Debug)]
/// Enum representing if the button is active low or active high.
pub enum ActiveState {
    /// Button is active low.
    ActiveLow,
    /// Button is active high.
    ActiveHigh,
}

#[derive(Debug)]
/// Struct representing a debouncer for a button.
pub struct Debouncer {
    integrator: u8,
    threshold: u8,
    sample_interval: Duration,
    active_state: ActiveState,
}

impl Debouncer {
    /// Creates a new Debouncer instance with the given threshold value.
    pub fn new(threshold: u8, sample_interval: Duration, active_state: ActiveState) -> Self {
        Self {
            integrator: 0,
            threshold,
            sample_interval,
            active_state,
        }
    }

    /// Debounces a button press using an integrator.
    pub async fn debounce<I: InputPin>(&mut self, gpio: &mut I) -> bool {
        loop {
            // Sample the button state
            let is_pressed = match self.active_state {
                ActiveState::ActiveLow => gpio.is_low().unwrap_or(false),
                ActiveState::ActiveHigh => gpio.is_high().unwrap_or(false),
            };

            if is_pressed {
                if self.integrator < self.threshold {
                    self.integrator += 1;
                }
            } else if self.integrator > 0 {
                self.integrator -= 1;
            }

            // Check if the integrator has crossed the threshold
            if self.integrator >= self.threshold {
                return true;
            } else if self.integrator == 0 {
                return false;
            }

            // Wait for the next sample interval
            Timer::after(self.sample_interval).await;
        }
    }
}

/// Default Debouncer with a threshold of 3 and sample interval of 10ms.
impl Default for Debouncer {
    fn default() -> Self {
        Self {
            integrator: 0,
            threshold: 3,
            sample_interval: Duration::from_millis(10),
            active_state: ActiveState::ActiveLow,
        }
    }
}
