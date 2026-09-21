#![no_std]
#![no_main]
use rp2040_hal::Clock;
use rp2040_hal::clocks::ClockSource;
use embedded_hal::digital::{InputPin, OutputPin};
use cortex_m_rt::entry;
use panic_halt as _;

use rp2040_hal::{
    clocks::init_clocks_and_plls,
    gpio::Pins,
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]

fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);


    let clocks = init_clocks_and_plls(
        12_000_000,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok().unwrap();

    let sio = Sio::new(pac.SIO);

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );



    let mut led = pins.gpio25.into_push_pull_output();
    let mut led2 = pins.gpio26.into_push_pull_output();
    let mut led3 = pins.gpio27.into_push_pull_output();
    let mut led4 = pins.gpio28.into_pull_up_input();
    let mut key = pins.gpio0.into_pull_up_input();
    let sda = pins.gpio4.into_function::<rp2040_hal::gpio::FunctionI2C();
    let scl = pins.gpio5.into_function::<rp2040_hal::gpio::FunctionI2C();
    let mut presses = 0;
    let mut progress = 0;
    
    loop {
        if key.is_low().unwrap(){
            presses+=1;
            progress+=10;

           
            led.set_high().unwrap();
            if presses == 1{
                led.set_high().unwrap()
            } elif presses == 2{
                led2.set_high().unwrap()
            } elif presses == 3{
                led3.set_high().unwrap()
            } elif presses == 4{
                led4.set_high().unwrap()
            }

            cortex_m::asm::delay(
                clocks.system_clock.freq().to_Hz() / (10 + presses.min(10))
            );

            use embedded_graphics::{
                mono_font::{ascii::FONT_6X10, MonoTextStyle},
                pixelcolor::BinaryColor,
                prelude::*,
                text::Text,
            };

            use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
            let interface = I2CDisplayInterface::new(i2c);

            let mut display = Ssd1306::new(
                interface,
                DisplaySize128x32,
                DisplayRotation::Rotate0,
            ).into_buffered_graphics_mode();


            let text_style = MonoTextStyle::new(&FONT_6X10,BinaryColor::On);


            let mut x = 0;
            let mut y = 0;

            let mut dx = 0;
            let mut dy = 0;

            
            display.clear(BinaryColor::Off).unwrap();

            Circle::new(Point::new(x,y), 8)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(&mut display)
                .unwrap();

            display.flush().unwrap();

            Rectangle::new(
                Point::new(12,10),
                Size::new(progress, 9)
            ).into_styled(
            PrimitiveStyle::with_fill(BinaryColor::On)
            ).draw(&mut display).unwrap();
            
            display.flush().unwrap();

            progress+=2;

            if progress>104{
                progress = 0;
            }

            cortex_m::asm::delay(
                clocks.system_clock.freq().to_Hz() / 60
            )

            x += dx;
            y += dy;

            if x <= 0|| x >= 120{
                dx = -dx;
            }
            if y<=0 || y>= 120{
                dy = -dy;
            }

            cortex_m::asm::delay(
                clocks.system_clock.freq().to_Hz() / 60 
            )
            Text::new(
                "Hello From {progress}!!",
                Point::new(10,30),
                text_style,
            ).draw(&mut display).unwrap();

            led.set_low().unwrap();
            led1.set_low().unwrap();
            led2.set_low().unwrap();
            led3.set_low().unwrap();
            let4.set_low().unwrap();


            while key.is_low().unwrap() {}

            cortex_m::asm::delay(
                clocks.system_clock.freq().to_Hz() / 100
            );
        }
    };
}




