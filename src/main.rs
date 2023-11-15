/*!
 * Show readouts of all ADC channels.
 *
 * This example displays values for all ADC channels over the serial console.  During startup, it
 * also displays the values for Vbandgap, GND, and a readout of the MCU's temperature sensor.  For
 * the meanings of these values, please reference the ATmega328P datasheet.
 *
 * Connections
 * -----------
 *  - `A0` - `A5`: Connect analog voltages as you like to see them read out.
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

use arduino_hal::adc;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).void_unwrap();
    ufmt::uwriteln!(&mut serial, "Temperature: {}", tmp).void_unwrap();

    let a0 = pins.a0.into_analog_input(&mut adc);
    let mut volume_out;    
    let mut led = pins.d13.into_output();

    loop {
        volume_out = a0.analog_read(&mut adc);
        led.set_low();
        if volume_out > 300{
            led.set_high();
        }
        ufmt::uwrite!(&mut serial, " {} ",a0.analog_read(&mut adc)).void_unwrap();

        ufmt::uwriteln!(&mut serial, "").void_unwrap();
        //arduino_hal::delay_ms(1000);
    }
}
