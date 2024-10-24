#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_imxrt::gpio::{self, Input, Polarity, Pull};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    pubsub::{PubSubChannel, Publisher, Subscriber},
};
use embedded_services::power_button::{Button, State};
use panic_probe as _;

#[derive(Clone, Copy)]
pub enum Message {
    BootToBootloader,
    EmergencyPowerOff,
    PowerButton(State),
    PowerOff,
    PowerOn,
}

/// Create a message bus.
static MESSAGE_BUS: PubSubChannel<ThreadModeRawMutex, Message, 4, 4, 4> = PubSubChannel::new();

#[embassy_executor::task(pool_size = 4)]
async fn button_task(gpio: Input<'static>, publisher: Publisher<'static, ThreadModeRawMutex, Message, 4, 4, 4>) {
    let mut button = Button::new(gpio);

    loop {
        let state = button.get_state().await;
        publisher.publish(Message::PowerButton(state)).await;
    }
}

#[embassy_executor::task]
async fn input_task(
    mut subscriber: Subscriber<'static, ThreadModeRawMutex, Message, 4, 4, 4>,
    publisher: Publisher<'static, ThreadModeRawMutex, Message, 4, 4, 4>,
) {
    let mut powered_on = false;

    loop {
        let msg = subscriber.next_message_pure().await;

        // TODO: Check other button presses

        if let Message::PowerButton(state) = msg {
            match state {
                State::NotPressed => {}
                State::Pressed(duration) => {
                    let duration = duration.as_millis();

                    match duration {
                        1..5000 => {
                            if powered_on {
                                powered_on = false;
                                publisher.publish(Message::PowerOff).await;
                            } else {
                                powered_on = true;
                                publisher.publish(Message::PowerOn).await;
                            }
                        }
                        5000..8000 => {
                            if powered_on {
                                powered_on = false;
                                publisher.publish(Message::PowerOff).await;
                            } else {
                                powered_on = true;
                                publisher.publish(Message::BootToBootloader).await;
                            }
                        }
                        _ => {
                            publisher.publish_immediate(Message::EmergencyPowerOff);
                        }
                    }
                }
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_imxrt::init(Default::default());

    unsafe { gpio::init() };

    // Create a power button instance
    let power_button = Input::new(p.PIO1_7, Pull::Up, Polarity::ActiveHigh);

    // TODO: Create other button instances

    spawner.must_spawn(button_task(power_button, MESSAGE_BUS.publisher().unwrap()));

    let mut subscriber = MESSAGE_BUS.subscriber().unwrap();

    loop {
        let msg = subscriber.next_message_pure().await;

        match msg {
            Message::BootToBootloader => {
                info!("Booting to bootloader");
            }
            Message::EmergencyPowerOff => {
                info!("Emergency Power Off");
            }
            Message::PowerOff => {
                info!("Power Off");
            }
            Message::PowerOn => {
                info!("Power On");
            }
            _ => {}
        }
    }
}
