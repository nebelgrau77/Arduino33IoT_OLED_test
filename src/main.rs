#![no_std]
#![no_main]

extern crate arduino_nano33iot as hal;
extern crate ssd1306;
extern crate arrayvec;
extern crate embedded_graphics;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::time::KiloHertz;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use embedded_graphics::{pixelcolor::BinaryColor,
                        prelude::*,
                        primitives::{Circle, Line, Rectangle},
                        style::PrimitiveStyleBuilder};

use ssd1306::{prelude::*, Builder as SSD1306Builder};

const BOOT_DELAY_MS: u16 = 100; 

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);    
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut led = pins.led_sck.into_open_drain_output(&mut pins.port);
    

    //delay for the I2C to initiate correctly and start on boot without having to reset the board
    delay.delay_ms(BOOT_DELAY_MS);

    let gclk0 = clocks.gclk0();
    let i2c = hal::sercom::I2CMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        KiloHertz(400),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        pins.sda,
        pins.scl,
        
    );

    let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x32).connect_i2c(i2c).into();
        
    disp.init().unwrap();
        
    disp.flush().unwrap();
   

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    Line::new(Point::new(8, 16 + 16), Point::new(8 + 16, 16 + 16))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    Line::new(Point::new(8, 16 + 16), Point::new(8 + 8, 16))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    Line::new(Point::new(8 + 16, 16 + 16), Point::new(8 + 8, 16))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    Rectangle::new(Point::new(48, 16), Point::new(48 + 16, 16 + 16))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    Circle::new(Point::new(96, 16 + 8), 8)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    
    
    loop {
        delay.delay_ms(200u8);
        led.set_high().unwrap();
        delay.delay_ms(200u8);
        led.set_low().unwrap();
    }
}



