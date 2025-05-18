# esp32c6-hdc3022
A basic binary crate for reading data from a HDC3022 with a ESP32-C6 DevKit

I have no idea what I'm doing, but at least this should work by simply running
"cargo run" with a `ESP32-C6-DevKitC 8MB` connected via USB-C and having the
I2C (SCL, SDA), 3.3V and ground pins connected to a HDC302x (tested with the
HDC3022).

Code heavily inspired by the sample code at
https://github.com/bobsrac/hdc302x-rs/tree/main/examples/esp32_embassy

# Physical setup
![Picture of the ESP32-C6 and HDC3022 connected to a breadboard this code was tested on.](/img/esp32c6-hdc3022.jpg)
