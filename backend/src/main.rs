use bb::{
    gpio::{
        GPIO,
        PinDirection
    },
    prelude::{
        DeviceState,
        PinState
    }
};
use libbeaglebone as bb;

fn main() {
    let mut led = GPIO::new(5);
    led.set_direction(PinDirection::Out).expect("TODO: panic message");
    led.set_export(DeviceState::Exported).expect("TODO: panic message");
    led.write(PinState::High).expect("TODO: panic message");
    println!("Hello, world!");
}

