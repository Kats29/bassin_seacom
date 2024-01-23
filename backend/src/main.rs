use std::{
    thread::{
        sleep,
    },
    time
};
use crate::backend::{
    DriverType,
    DriverCN
};

mod backend;

fn main() -> sysfs_gpio::Result<()> {
    let mut driver = backend::DriverCN::new(true, DriverType::X).unwrap();
    // driver.go()?;
    println!("Hello, world! De Goulven");
    let mut value = false;
    loop {
        sleep(time::Duration::from_millis(1000));
        value = !value;
    };
}

