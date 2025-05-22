//! Low-level example of external messaging with a simple type-C service
use embassy_executor::{Executor, Spawner};
use embedded_services::type_c::{external, ControllerId};
use embedded_usb_pd::GlobalPortId;
use log::*;
use static_cell::StaticCell;

#[embassy_executor::task]
async fn task(_spawner: Spawner) {
    info!("Starting main task");
    embedded_services::init().await;

    info!("Getting controller status");
    let controller_status = external::get_controller_status(ControllerId(0)).await.unwrap();
    info!("Controller status: {:?}", controller_status);

    info!("Getting port status");
    let port_status = external::get_port_status(GlobalPortId(0)).await.unwrap();
    info!("Port status: {:?}", port_status);

    info!("Getting retimer fw update status");
    let rt_fw_update_status = external::port_get_rt_fw_update_status(GlobalPortId(0)).await.unwrap();
    info!("Port status: {:?}", port_status);

    info!("Setting retimer fw update state");
    let cmd_state = external::port_set_rt_fw_update_state(GlobalPortId(0)).await.unwrap();
    info!("Set retimer fw update state: {:?}", cmd_state);

    info!("Clearing retimer fw update state");
    let cmd_state = external::get_port_status(GlobalPortId(0)).await.unwrap();
    info!("Clear retimer fw update state: {:?}", cmd_state);
}

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();

    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.must_spawn(type_c_service::task());
        spawner.must_spawn(task(spawner));
    });
}
