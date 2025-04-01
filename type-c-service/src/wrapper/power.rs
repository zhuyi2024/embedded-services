//! Module contain power-policy related message handling
use embedded_services::power::policy::device::RequestData;

use super::*;

impl<const N: usize, C: Controller> ControllerWrapper<'_, N, C> {
    /// Return the power device for the given port
    pub(super) fn get_power_device(&self, port: LocalPortId) -> Result<&policy::device::Device, Error<C::BusError>> {
        if port.0 > N as u8 {
            return PdError::InvalidPort.into();
        }
        Ok(&self.power[port.0 as usize])
    }

    /// Handle a new consumer contract
    pub(super) async fn process_new_consumer_contract(
        &self,
        power: &policy::device::Device,
        status: &PortStatus,
    ) -> Result<(), Error<C::BusError>> {
        info!("New consumer contract");

        if let Some(contract) = status.contract {
            if !matches!(contract, Contract::Sink(_)) {
                error!("Not a sink contract");
                return PdError::InvalidMode.into();
            }
        } else {
            error!("No contract");
            return PdError::InvalidMode.into();
        }

        let contract = status.contract.unwrap();
        let current_state = power.state().await.kind();
        // Don't update the available consumer contract if we're providing power
        if current_state != StateKind::ConnectedProvider {
            // Recover if we're not in the correct state
            if let action::device::AnyState::Detached(state) = power.device_action().await {
                if let Err(e) = state.attach().await {
                    error!("Error attaching power device: {:?}", e);
                    return PdError::Failed.into();
                }
            }

            if let Ok(state) = power.try_device_action::<action::Idle>().await {
                if let Err(e) = state
                    .notify_consumer_power_capability(Some(policy::PowerCapability::from(contract)))
                    .await
                {
                    error!("Error setting power contract: {:?}", e);
                    return PdError::Failed.into();
                }
            } else if let Ok(state) = power.try_device_action::<action::ConnectedConsumer>().await {
                if let Err(e) = state
                    .notify_consumer_power_capability(Some(policy::PowerCapability::from(contract)))
                    .await
                {
                    error!("Error setting power contract: {:?}", e);
                    return PdError::Failed.into();
                }
            } else {
                error!("Power device not in detached state");
                return PdError::InvalidMode.into();
            }
        }

        Ok(())
    }

    /// Wait for a power command
    pub(super) async fn wait_power_command(&self) -> (RequestData, LocalPortId) {
        let futures: [_; N] = from_fn(|i| self.power[i].wait_request());

        let (command, local_id) = select_array(futures).await;
        trace!("Power command: device{} {:#?}", local_id, command);
        (command, LocalPortId(local_id as u8))
    }

    /// Process a power command
    /// Returns no error because this is a top-level function
    pub(super) async fn process_power_command(&self, controller: &mut C, port: LocalPortId, command: RequestData) {
        trace!("Processing power command: device{} {:#?}", port.0, command);
        let power = match self.get_power_device(port) {
            Ok(power) => power,
            Err(_) => {
                error!("Port{}: Error getting power device for port", port.0);
                return;
            }
        };

        match command {
            policy::device::RequestData::ConnectConsumer(capability) => {
                info!("Port{}: Connect consumer: {:?}", port.0, capability);
                if controller.enable_sink_path(port, true).await.is_err() {
                    error!("Error enabling sink path");
                    power.send_response(Err(policy::Error::Failed)).await;
                    return;
                }
            }
            policy::device::RequestData::Disconnect => {
                info!("Port{}: Disconnect", port.0);
                if controller.enable_sink_path(port, false).await.is_err() {
                    error!("Error disabling sink path");
                    power.send_response(Err(policy::Error::Failed)).await;
                    return;
                }
            }
            _ => {}
        }

        power.send_response(Ok(policy::device::ResponseData::Complete)).await;
    }
}
