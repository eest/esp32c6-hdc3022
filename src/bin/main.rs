#![no_std]
#![no_main]

use embassy_time::{Duration, Timer};
use esp_hal::{
    clock::CpuClock,
    i2c::master,
    time::Rate,
    timer::{systimer::SystemTimer, timg::TimerGroup, OneShotTimer},
};
use esp_println::println;
use hdc302x_async::{Hdc302x, I2cAddr, LowPowerMode, ManufacturerId};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_s: embassy_executor::Spawner) {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let systimer = SystemTimer::new(peripherals.SYSTIMER);
    let alarm0 = systimer.alarm0;
    let delay = OneShotTimer::new(alarm0).into_async();

    let i2c = master::I2c::new(
        peripherals.I2C0,
        master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO6)
    .with_scl(peripherals.GPIO7)
    .into_async();

    let mut hdc302x = Hdc302x::new(i2c, delay, I2cAddr::Addr00);

    match hdc302x.read_manufacturer_id().await {
        Ok(ManufacturerId::TexasInstruments) => {
            println!(
                "hdc302x: manufacturer id: {}",
                ManufacturerId::TexasInstruments
            );
        }
        Ok(manuf_id) => {
            println!("hdc302x: unexpected manufacturer id: {manuf_id}");
            return;
        }
        Err(e) => {
            println!("hdc302x: read_manufacturer_id error: {e:?}");
            return;
        }
    }

    match hdc302x.read_serial_number().await {
        Ok(serial_number) => {
            println!("hdc302x: serial_number: {serial_number}");
        }
        Err(e) => {
            println!("hdc302x: read_serial_number error: {e:?}");
            return;
        }
    }

    match hdc302x.read_status(true).await {
        Ok(status_bits) => {
            println!("hdc302x: status_bits: {status_bits}");
        }
        Err(e) => {
            println!("hdc302x: read_status error: {e:?}");
            return;
        }
    }

    loop {
        let raw_datum = hdc302x
            .one_shot(LowPowerMode::lowest_noise())
            .await
            .unwrap();

        let d = hdc302x_async::Datum::from(&raw_datum);
        println!("{:?}", d);

        Timer::after(Duration::from_millis(1_000)).await;
    }
}
