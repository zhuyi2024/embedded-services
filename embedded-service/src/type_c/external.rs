//! Message definitions for external type-C commands
use embedded_usb_pd::{GlobalPortId, PdError, PortId as LocalPortId};

use super::{
    controller::{
        execute_external_controller_command, execute_external_port_command, lookup_controller, ControllerStatus,
        PortStatus,
    },
    ControllerId,
};

/// Data for controller-specific commands
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ControllerCommandData {
    /// Get controller status
    ControllerStatus,
}

/// Controller-specific commands
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControllerCommand {
    /// Controller ID
    pub id: ControllerId,
    /// Command data
    pub data: ControllerCommandData,
}

/// Response data for controller-specific commands
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ControllerResponseData<'a> {
    /// Get controller status
    ControllerStatus(ControllerStatus<'a>),
}

/// Controller-specific command response
pub type ControllerResponse<'a> = Result<ControllerResponseData<'a>, PdError>;

/// Data for port-specific commands
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PortCommandData {
    /// Get port status
    PortStatus,
    /// Get retimer fw update status
    RetimerFwUpdateGetState,
    /// Set retimer fw update status
    RetimerFwUpdateSetState,
    /// Clear retimer fw update status
    RetimerFwUpdateClearState,
}

/// Port-specific commands
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PortCommand {
    /// Port ID
    pub port: GlobalPortId,
    /// Command data
    pub data: PortCommandData,
}

/// Response data for port-specific commands
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PortResponseData {
    /// Command completed with no error
    Complete,
    /// Get port status
    PortStatus(PortStatus),
    /// Get retimer fw update status
    RetimerFwUpdateGetState(bool),
}

/// Port-specific command response
pub type PortResponse = Result<PortResponseData, PdError>;

/// External commands for type-C service
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Command {
    /// Port command
    Port(PortCommand),
    /// Controller command
    Controller(ControllerCommand),
}

/// External command response for type-C service
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Response<'a> {
    /// Port command response
    Port(PortResponse),
    /// Controller command response
    Controller(ControllerResponse<'a>),
}

/// Get the status of the given port
#[allow(unreachable_patterns)]
pub async fn get_port_status(port: GlobalPortId) -> Result<PortStatus, PdError> {
    match execute_external_port_command(Command::Port(PortCommand {
        port,
        data: PortCommandData::PortStatus,
    }))
    .await?
    {
        PortResponseData::PortStatus(status) => Ok(status),
        _ => Err(PdError::InvalidResponse),
    }
}

/// Get the status of the given port by its controller and local port ID
pub async fn get_controller_port_status(controller: ControllerId, port: LocalPortId) -> Result<PortStatus, PdError> {
    let global_port = controller_port_to_global_id(controller, port).await?;
    get_port_status(global_port).await
}

/// Get the status of the given controller
#[allow(unreachable_patterns)]
pub async fn get_controller_status(id: ControllerId) -> Result<ControllerStatus<'static>, PdError> {
    match execute_external_controller_command(Command::Controller(ControllerCommand {
        id,
        data: ControllerCommandData::ControllerStatus,
    }))
    .await?
    {
        ControllerResponseData::ControllerStatus(status) => Ok(status),
        _ => Err(PdError::InvalidResponse),
    }
}

/// Get the number of ports on the given controller
pub async fn get_controller_num_ports(controller_id: ControllerId) -> Result<usize, PdError> {
    Ok(lookup_controller(controller_id).await?.num_ports())
}

/// Convert a (controller ID, local port ID) to a global port ID
pub async fn controller_port_to_global_id(
    controller_id: ControllerId,
    port_id: LocalPortId,
) -> Result<GlobalPortId, PdError> {
    lookup_controller(controller_id).await?.lookup_global_port(port_id)
}

/// Get the retimer fw update status of the given port
#[allow(unreachable_patterns)]
pub async fn port_get_rt_fw_update_status(port: GlobalPortId) -> Result<bool, PdError> {
    match execute_external_port_command(Command::Port(PortCommand {
        port,
        data: PortCommandData::RetimerFwUpdateGetState,
    }))
    .await?
    {
        PortResponseData::RetimerFwUpdateGetState(status) => Ok(status),
        _ => Err(PdError::InvalidResponse),
    }
}

/// Set the retimer fw update state of the given port
#[allow(unreachable_patterns)]
pub async fn port_set_rt_fw_update_state(port: GlobalPortId) -> Result<(), PdError> {
    match execute_external_port_command(Command::Port(PortCommand {
        port,
        data: PortCommandData::RetimerFwUpdateSetState,
    }))
    .await?
    {
        PortResponseData::Complete => Ok(()),
        _ => Err(PdError::InvalidResponse),
    }
}

/// Clear the retimer fw update state of the given port
#[allow(unreachable_patterns)]
pub async fn port_clear_rt_fw_update_state(port: GlobalPortId) -> Result<(), PdError> {
    match execute_external_port_command(Command::Port(PortCommand {
        port,
        data: PortCommandData::RetimerFwUpdateSetState,
    }))
    .await?
    {
        PortResponseData::Complete => Ok(()),
        _ => Err(PdError::InvalidResponse),
    }
}
