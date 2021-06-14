#![no_std]
#![no_main]

use esp32_hal::target;
use hal::prelude::*;
use xtensa_lx::timer::delay;
use panic_halt as _;
use esp32_hal as hal;

const CORE_HZ: u32 = 40_000_000;
const WDT_WKEY_VALUE: u32 = 0x50D83AA1;

#[entry]
fn main() -> ! {
  let dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

  let pins = dp.GPIO.split();
  let mut led = pins.gpio0.into_push_pull_output();
  loop {
    led.set_high().unwrap();
    delay(CORE_HZ);
    led.set_low().unwrap();
    delay(CORE_HZ);
  }
}
