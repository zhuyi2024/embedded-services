#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_imxrt::gpio::{self, Input, Inverter, Pull};
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    pubsub::{PubSubChannel, Publisher},
};
use embassy_time::Duration;
use embedded_services::{
    button::Button,
    button_interpreter::{check_button_press, Message},
    debounce::{ActiveState, Debouncer},
};
use panic_probe as _;

/// Create a message bus.
static MESSAGE_BUS: PubSubChannel<ThreadModeRawMutex, Message, 4, 4, 4> = PubSubChannel::new();

#[embassy_executor::task(pool_size = 4)]
async fn button_task(
    gpio: Input<'static>,
    debouncer: Debouncer,
    publisher: Publisher<'static, ThreadModeRawMutex, Message, 4, 4, 4>,
) {
    let mut button = Button::new(gpio, debouncer);

    loop {
        match check_button_press(&mut button).await {
            Message::ShortPress => {
                info!("Short press");
                publisher.publish(Message::ShortPress).await;
            }
            Message::LongPress => {
                info!("Long press");
                publisher.publish(Message::LongPress).await;
            }
            Message::PressAndHold => {
                info!("Press and hold");
                publisher.publish(Message::PressAndHold).await;
            }
            Message::Ignore => {
                // info!("Ignore");
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_imxrt::init(Default::default());

    unsafe { gpio::init() };

    // Create a power button instance
    let power_button = Input::new(p.PIO1_1, Pull::Up, Inverter::Disabled);

    // Create a debouncer instance
    let debouncer = Debouncer::new(3, Duration::from_millis(10), ActiveState::ActiveLow);

    spawner.must_spawn(button_task(power_button, debouncer, MESSAGE_BUS.publisher().unwrap()));

    // Create an LED instance
    let mut led_r = gpio::Output::new(
        p.PIO0_31,
        gpio::Level::Low,
        gpio::DriveMode::PushPull,
        gpio::DriveStrength::Normal,
        gpio::SlewRate::Standard,
    );

    // Create an LED instance
    let mut led_g = gpio::Output::new(
        p.PIO0_14,
        gpio::Level::Low,
        gpio::DriveMode::PushPull,
        gpio::DriveStrength::Normal,
        gpio::SlewRate::Standard,
    );

    // Create an LED instance
    let mut led_b = gpio::Output::new(
        p.PIO0_26,
        gpio::Level::Low,
        gpio::DriveMode::PushPull,
        gpio::DriveStrength::Normal,
        gpio::SlewRate::Standard,
    );

    let mut subscriber = MESSAGE_BUS.subscriber().unwrap();

    loop {
        let msg = subscriber.next_message_pure().await;

        match msg {
            Message::ShortPress => {
                led_g.toggle();
            }
            Message::LongPress => {
                led_b.toggle();
            }
            Message::PressAndHold => {
                led_r.toggle();
            }
            _ => {}
        }
    }
}
