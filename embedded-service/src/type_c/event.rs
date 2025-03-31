//! Event definitions
use bitflags::bitflags;
use bitvec::BitArr;
use embedded_usb_pd::GlobalPortId;

/// Port event kind
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PortEventKind(pub u32);

bitflags! {
    impl PortEventKind: u32 {
         /// None
        const NONE = 0;
        /// Plug inserted or removed
        const PLUG_INSERTED_OR_REMOVED = (1 << 0);
        /// New contract as consumer
        const NEW_POWER_CONTRACT_AS_CONSUMER = (1 << 3);
    }
}

impl PortEventKind {
    /// Returns true if a plug was inserted or removed
    pub fn plug_inserted_or_removed(self) -> bool {
        self & Self::PLUG_INSERTED_OR_REMOVED != Self::NONE
    }

    /// Returns true if a new power contract was established as consumer
    pub fn new_power_contract_as_consumer(self) -> bool {
        self & Self::NEW_POWER_CONTRACT_AS_CONSUMER != Self::NONE
    }
}

/// Bit vector type to store pending port events
type PortEventFlagsVec = BitArr!(for 32, in u32);

/// Pending port events
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PortEventFlags(PortEventFlagsVec);

impl PortEventFlags {
    /// Creates a new PortEventFlags with no pending events
    pub const fn none() -> Self {
        Self(PortEventFlagsVec::ZERO)
    }

    /// Returns true if there are no pending events
    pub fn is_none(&self) -> bool {
        self.0 == PortEventFlagsVec::ZERO
    }

    /// Marks the given port as pending
    pub fn pend_port(&mut self, port: GlobalPortId) {
        self.0.set(port.0 as usize, true);
    }

    /// Returns true if the given port is pending
    pub fn is_pending(&self, port: GlobalPortId) -> bool {
        self.0[port.0 as usize]
    }

    /// Returns a combination of the current event flags and other
    pub fn union(&self, other: PortEventFlags) -> PortEventFlags {
        PortEventFlags(self.0 | other.0)
    }

    /// Returns the number of bits in the event
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<PortEventFlags> for u32 {
    fn from(flags: PortEventFlags) -> Self {
        flags.0.data[0]
    }
}
