//! Definitions for deferred execution of commands
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::debug;
use embassy_sync::{blocking_mutex::raw::RawMutex, mutex::Mutex, signal::Signal};

/// A unique identifier for a particular command invocation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
struct RequestId(usize);

/// A simple channel for executing deferred commands.
///
/// This implementation provides synchronization for command invocations
/// and ensures that responses are sent back to the correct sender
/// using a unique invocation ID.
pub struct Channel<M: RawMutex, C, R> {
    /// Signal for sending commands
    command: Signal<M, (C, RequestId)>,
    /// Signal for receiving responses
    response: Signal<M, (R, RequestId)>,
    /// Mutex for synchronizing access to command invocation
    request_lock: Mutex<M, ()>,
    /// Unique ID for the next invocation
    next_request_id: AtomicUsize,
}

impl<M: RawMutex, C, R> Channel<M, C, R> {
    /// Create a new channel
    pub const fn new() -> Self {
        Self {
            command: Signal::new(),
            response: Signal::new(),
            request_lock: Mutex::new(()),
            next_request_id: AtomicUsize::new(0),
        }
    }

    /// Get the next request ID
    fn get_next_request_id(&self) -> RequestId {
        let id = self.next_request_id.fetch_add(1, Ordering::SeqCst);
        RequestId(id)
    }

    /// Send a command and return the response
    /// This locks to ensure that commands are executed atomically
    pub async fn execute(&self, command: C) -> R {
        let _guard = self.request_lock.lock().await;
        let request_id = self.get_next_request_id();
        self.command.signal((command, request_id));
        loop {
            // Wait until we receive a response for out particular request
            let (response, id) = self.response.wait().await;
            if id == request_id {
                return response;
            } else {
                // Not an error because this is the expected behavior in certain cases,
                // particularly if the sender times out before the response is received.
                debug!("Received response for different invocation: {}", id.0);
            }
        }
    }

    /// Wait for an invocation
    pub async fn receive(&self) -> Request<'_, M, C, R> {
        let (command, request_id) = self.command.wait().await;
        Request {
            channel: self,
            request_id,
            command,
        }
    }
}

impl<M: RawMutex, C, R> Default for Channel<M, C, R> {
    /// Default implementation
    fn default() -> Self {
        Self::new()
    }
}

/// A specific request
pub struct Request<'a, M: RawMutex, C, R> {
    /// The channel this invocation came from
    channel: &'a Channel<M, C, R>,
    /// Request ID
    request_id: RequestId,
    /// Command to execute
    pub command: C,
}

impl<M: RawMutex, C, R> Request<'_, M, C, R> {
    /// Send a response to the command, consuming the command in the process.
    ///
    /// Consuming the command ensures each command may only be responded to once.
    pub fn respond(self, response: R) {
        self.channel.response.signal((response, self.request_id));
    }
}
