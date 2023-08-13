#![no_std]
#![no_main] // bsp-rt is used as the entry point of the program instead
#![feature(type_alias_impl_trait)] // this feature is needed for RTIC v2
#![allow(unused_imports)]

//// BASIC BSP PACKAGES ///
use bsp::board;
use teensy4_bsp as bsp;
use teensy4_panic as _; // allows program to panic and print panic messages
//////////////////////////

//// ASSOCIATED TPYES FOR INSTANCES ////
use teensy4_pins::common::*; // pad to pin definitions
use bsp::hal::gpio; // gpio module
////////////////////////////////////////

//// RTIC PKACAGES ///
use rtic::app;
use rtic_monotonics::systick::*;
////////////////////

// local example driver
//use sx127x_lora;

// spi traits to use the transfer and write transactions
use embedded_hal::blocking::spi::{Transfer, Write};

//// THE APP MODULE ////
//// device: board support package
//// perihperals: ...?
//// dispatchers: interrupt handlers for software defined tasks
#[app(device = bsp, peripherals = true, dispatchers = [GPT1])]
mod app {
    // this allows us to define our packages outside the app module
    // we're essetially "bringing them all in"
    use super::*;

    // accounts for our syst_clock to be in 10 kHz (normal is 1 kHz)
    // this means that the granularity for the delay is 0.1 ms per tick
    // therefore we multiply our delay time by a factor of 10
    const SYST_MONO_FACTOR: u32 = 10;

    // delay in miliseconds
    const DELAY_MS: u32 = SYST_MONO_FACTOR * 3000;

    // type definition for Led :)
    // this simplifies local and shared resource definitions
    type Led = gpio::Output<P8>;

    // struct that holds local resources which can be accessed via the context
    #[local]
    struct Local {
        spi: board::Lpspi4,
        led: Led,
    }

    // struct that holds shared resources which can be accessed via the context
    #[shared]
    struct Shared {
        counter: u32,
    }

    // entry point of the "program"
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // allocate the resources needed
        let board::Resources {
            // usedd to acces pin names
            pins,
            // used to control any pin from the gpio2 register
            // (e.g. pin13 for the on board LED)
            mut gpio2,
            // for usb logging :)
            usb,
            // resources to control spi
            lpspi4,
            ..
        } = board::t40(cx.device);

        // usb logging setup
        bsp::LoggingFrontend::default_log().register_usb(usb);

        // systick monotonic setup
        let systick_token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 36_000_000, systick_token);

        // init led from gpio2 pin 14
        let led = gpio2.output(pins.p8);

        // init counter shared variable
        let counter = 0;

        // initalize spi
        let spi = board::lpspi(lpspi4, 
            board::LpspiPins {
                pcs0: pins.p10,
                sck: pins.p13,
                sdo: pins.p11,
                sdi: pins.p12,
            }, 
            1_000_000);

        // spawn a toggle call
        toggle::spawn().unwrap();

        // spawn the radio task
        talk_to_radio::spawn().unwrap();

        // return the local, and shared resources to be used from the context
        (
            Shared {counter},
            Local {spi, led}
        )
    }

    // lowest priority tasks that runs only while no other task is running
    #[idle]
    fn idle(_: idle::Context) -> !{
        loop {
            // wfi: wait-for-interrupt
            cortex_m::asm::wfi();
        }
    }

    #[task(local = [led], shared = [counter], priority = 1)]
    async fn toggle(cx : toggle::Context) {
        // just renaming our shared variable into a local variable so it's easier to read
        let mut counter = cx.shared.counter;

        // infinite loop which is allowed as it contains a delay followed by a ".await"
        loop {
            // example locking the shared counter variable and updating it's value!
            counter.lock(|counter| {
                // increment the counter using an external function
                *counter += 1;
                
                // prints "blink!" to the usb serial port
                log::info!("blink # {}!", *counter);
            });

            // toggle the led
            cx.local.led.toggle();

            // generate a delay using the initialized systick monotonic
            // by calling the Systick::delay() function
            Systick::delay(DELAY_MS.millis()).await;
        }

    }

    // DUMMY TASK
    // Used to test if pcs0 is automatically asserted via hardware on every Transaction
    // According to the IMXRT HAL Documentation this is the intended behaviour
    #[task(local = [spi], priority = 1)]
    async fn talk_to_radio(cx: talk_to_radio::Context) {
        let spi = cx.local.spi;

            Systick::delay(DELAY_MS.millis()).await;
            log::info!("asking for radio's version...");

            // Perform a Transfer which is a full duplex transaction
            // on register address 0x42 (which contains the radio's version)
            // in practice, this is a hardware validity check
            let reg: u8 = 0x42;
            let mut buffer = [reg & 0x7f, 0];

            let result = spi.transfer(&mut buffer).unwrap();

            // print ther result onto the usb serial port
            log::info!("radio's version is: {:#04x}. Expected version: 0x12", result[1] as u8);
    }

}

