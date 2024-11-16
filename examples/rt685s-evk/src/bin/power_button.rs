#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_imxrt::gpio::{self, Input, Inverter, Pull};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::pubsub::{PubSubChannel, Publisher};
use embassy_time::Duration;
use embedded_services::power_button::button::{Button, ButtonConfig};
use embedded_services::power_button::button_interpreter::{check_button_press, Message};
use embedded_services::power_button::debounce::{ActiveState, Debouncer};
use {defmt_rtt as _, panic_probe as _};

/// Create a message bus.
static MESSAGE_BUS: PubSubChannel<ThreadModeRawMutex, Message, 4, 4, 4> = PubSubChannel::new();

#[embassy_executor::task(pool_size = 4)]
async fn button_task(
    gpio: Input<'static>,
    config: ButtonConfig,
    publisher: Publisher<'static, ThreadModeRawMutex, Message, 4, 4, 4>,
) {
    let mut button = Button::new(gpio, config);

    loop {
        match check_button_press(&mut button).await {
            Some(Message::ShortPress) => {
                info!("Short press");
                publisher.publish(Message::ShortPress).await;
            }
            Some(Message::LongPress) => {
                info!("Long press");
                publisher.publish(Message::LongPress).await;
            }
            Some(Message::PressAndHold) => {
                info!("Press and hold");
                publisher.publish(Message::PressAndHold).await;
            }
            None => {}
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_imxrt::init(Default::default());

    unsafe { gpio::init() };

    // Create a power button instance
    let button_a = Input::new(p.PIO1_1, Pull::Up, Inverter::Disabled);
    // Create a debouncer instance
    let debouncer = Debouncer::new(3, Duration::from_millis(10), ActiveState::ActiveLow);
    // Create a custom button configuration instance
    let config_a = ButtonConfig::new(debouncer, Duration::from_millis(1000), Duration::from_millis(2000));

    // Create a second button instance
    let button_b = Input::new(p.PIO0_10, Pull::Up, Inverter::Disabled);
    // Create a default button configuration instance
    let config_b = ButtonConfig::default();

    // Spawn the button tasks
    spawner.must_spawn(button_task(button_a, config_a, MESSAGE_BUS.publisher().unwrap()));
    spawner.must_spawn(button_task(button_b, config_b, MESSAGE_BUS.publisher().unwrap()));

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

        // Toggle the LEDs based on the button press duration
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
        }
    }
}
