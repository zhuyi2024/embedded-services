//! Comms service message definitions

use embedded_usb_pd::GlobalPortId;

use super::event::PortNotificationSingle;

/// Message generated when a debug acessory is connected or disconnected
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DebugAccessoryMessage {
    /// Port
    pub port: GlobalPortId,
    /// Connected
    pub connected: bool,
}

/// Message generated when a port notification occurs
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PortNotificationMessage {
    /// Port
    pub port: GlobalPortId,
    /// notification signal
    pub notification: PortNotificationSingle,
}
