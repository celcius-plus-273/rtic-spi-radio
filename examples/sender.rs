#![no_std]
#![no_main] // bsp-rt is used as the entry point of the program instead
#![feature(type_alias_impl_trait)] // this feature is needed for RTIC v2

//// BASIC BSP PACKAGES ///
use bsp::board;
use teensy4_bsp as bsp;
use teensy4_panic as _; // allows program to panic and print panic messages
//////////////////////////

//// ASSOCIATED TPYES FOR INSTANCES ////
use teensy4_pins::common::*; // pad to pin definitions
use bsp::hal::gpio::{self, Trigger}; // gpio module
use bsp::hal::gpt::Gpt1; // tpye definition for GPT1
use bsp::hal::timer::Blocking;
////////////////////////////////////////

//// RTIC PKACAGES ///
use rtic::app;
use rtic_monotonics::systick::*;
////////////////////

// local example driver
use sx127x_lora::{self, RadioMode};

// spi traits to use the transfer and write transactions
use embedded_hal::blocking::spi::{Transfer, Write};
use bsp::hal::gpt::ClockSource;

//// THE APP MODULE ////
//// device: board support package
//// perihperals: ...?
//// dispatchers: interrupt handlers for software defined tasks
#[app(
    device = bsp, 
    peripherals = true, 
    dispatchers = [GPT2]
)]
mod app {

    // this allows us to define our packages outside the app module
    // we're essetially "bringing them all in"
    use super::*;

    // accounts for our syst_clock to be in 10 kHz (normal is 1 kHz)
    // this means that the granularity for the delay is 0.1 ms per tick
    // therefore we multiply our delay time by a factor of 10
    const SYST_MONO_FACTOR: u32 = 10;

    // delay in miliseconds
    const DELAY_MS: u32 = SYST_MONO_FACTOR * 2000;
    const LONG_DELAY: u32 = SYST_MONO_FACTOR * 10_000;

    // FREQUENCY for LoRa
    const FREQUENCY: i64 = 915;

    // timer stuff
    const GPT1_FREQUENCY: u32 = 1_000;
    const GPT1_CLOCK_SOURCE: ClockSource = ClockSource::HighFrequencyReferenceClock;
    const GPT1_DIVIDER: u32 = board::PERCLK_FREQUENCY / GPT1_FREQUENCY;

    // type definition for Led :)
    // this simplifies local and shared resource definitions
    type Led = gpio::Output<P7>;
    type Delay = Blocking<Gpt1, GPT1_FREQUENCY>;
    type Interrupt = gpio::Input<P1>;
    type Lora_Radio = sx127x_lora::LoRa<board::Lpspi4, gpio::Output<P8>, gpio::Output<P9>, Delay>;

    // struct that holds local resources which can be accessed via the context
    #[local]
    struct Local {
        led: Led,
        rxInt: Interrupt,
    }

    // struct that holds shared resources which can be accessed via the context
    #[shared]
    struct Shared {
        counter: u32,
        lora: Lora_Radio,
    }

    // entry point of the "program"
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // allocate the resources needed
        let board::Resources {
            // usedd to acces pin names
            pins,
            // gpio1 for pin 1
            mut gpio1,
            // used to control any pin from the gpio2 register
            // (e.g. pin13 for the on board LED)
            mut gpio2,
            // for usb logging :)
            usb,
            // resources to control spi
            lpspi4,
            // for blocking delays :)
            mut gpt1,
            ..
        } = board::t40(cx.device);

        // usb logging setup
        bsp::LoggingFrontend::default_log().register_usb(usb);

        // systick monotonic setup
        let systick_token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 36_000_000, systick_token);

        // gpt1 as blocking delay
        gpt1.disable();
        gpt1.set_divider(GPT1_DIVIDER);
        gpt1.set_clock_source(GPT1_CLOCK_SOURCE);
        let mut delay = Blocking::<_, GPT1_FREQUENCY>::from_gpt(gpt1);

        // RX DONE Interrupt setup
        let rxInt = gpio1.input(pins.p1);
        gpio1.set_interrupt(&rxInt, Some(Trigger::RisingEdge));

        // init led from gpio2 pin 7
        let led = gpio2.output(pins.p7);

        // init counter shared variable
        let counter = 0;

        // initalize spi
        let mut spi = board::lpspi(lpspi4, 
            board::LpspiPins {
                pcs0: pins.p10,
                sck: pins.p13,
                sdo: pins.p11,
                sdi: pins.p12,
            }, 
            1_000_000);

        // init fake CS pin (TEMPORARY) and required reset pin
        let fake_cs = gpio2.output(pins.p8);
        let reset = gpio2.output(pins.p9);

        // initialize lora module
        let mut lora = match sx127x_lora::LoRa::new(spi, fake_cs, reset, FREQUENCY, delay) {
            Ok(sx127x) => sx127x,
            Err(error) => match error {
                sx127x_lora::Error::VersionMismatch(version) => panic!("Version Mismatch Error. Version{:?}", version),
                sx127x_lora::Error::CS(_) => panic!("Chip select issue"),
                sx127x_lora::Error::Reset(_) => panic!("Reset issue"),
                sx127x_lora::Error::SPI(_) => panic!("SPI problem"),
                sx127x_lora::Error::Transmitting => panic!("Error during spi transmission"),
                sx127x_lora::Error::Uninformative => panic!("Uninformative error RIP"),
            }
        };

        // spawn a toggle call
        toggle::spawn().unwrap();

        // spawn the radio task
        init_radio::spawn().unwrap();

        // return the local, and shared resources to be used from the context
        (
            Shared {counter, lora},
            Local {led, rxInt}
        )
    }

    // (optional) lowest priority tasks that runs only while no other task is running
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

    // Although LoRa is already initalized in the INIT context we also set some settings
    // on this software task
    // FOR THIS TEST: THIS ALSO SETS LORA IN RXCONTINUOUS MODE
    #[task(shared = [lora], priority = 1)]
    async fn init_radio(cx: init_radio::Context) {
        let mut lora = cx.shared.lora;

        Systick::delay(DELAY_MS.millis()).await;

        lora.lock(|lora| {
            log::info!("lora's version is: {:#04x}", lora.get_radio_version().unwrap());

            match lora.set_tx_power(17, 1) {
                Ok(_) => log::info!("Succesfully set TX power"),
                Err(_) => panic!("Error setting tx power"),
            };

            log::info!("Succesfully init LoRa!");

            //////////////////////////////////////////////////////////////////////
            //// We're testing Transmitting so no need to setup as receive :) ////
            //////////////////////////////////////////////////////////////////////
            // match lora.set_mode(RadioMode::RxContinuous) {
            //     Ok(_) => log::info!("Lora is listening..."),
            //     Err(_) => panic!("Couldn't set radio to RxContinuos"),
            // };
            //////////////////////////////////////////////////////////////////////
        });

        // calls the sender task
        sender::spawn().unwrap();
    }

    #[task(shared = [lora], priority = 1)]
    async fn sender(cx: sender::Context) {
        let mut lora = cx.shared.lora;

        // pack the message into an u8 array
        let message = "IT WORKS!!";
        let mut buffer = [0;255];
        for (i,c) in message.chars().enumerate() {
            buffer[i] = c as u8;
        }

        loop {
            // transmit using blocking function
            log::info!("Sending... message: {message}");

            lora.lock(|lora| {
                match lora.transmit_payload_busy(buffer, message.len()) {
                    Ok(packet_size) => log::info!("Sent packet with size: {}", packet_size),
                    Err(_) => panic!("Error sending packet x.x"),
                };
            });
            
            Systick::delay(LONG_DELAY.millis()).await;
        }
    }

    // RECEIVE VIA INTERRUPT!
    #[task(binds = GPIO1_COMBINED_0_15, local = [rxInt], shared = [lora])]
    fn receive_data(cx: receive_data::Context) {
        let mut lora = cx.shared.lora;

        /// Interrupt is being triggered twice... not sure why
        /// maybe just add a boolean check?
        /// When adding a slight delaya it doesn't get triggered twice anymore...
        /// I think the radio is taking some time itself to clear the RxDone irq
        /// Added a slight delay to compensate for that
        log::info!("Interrupt triggered!");
        
        // read the data in the buffer
        lora.lock(|lora| {
            match lora.read_packet() {
                Ok(buffer) => for c in buffer {
                                if c == 0x0 {break};
                                log::info!("got: {}", c as char);
                            },
                Err(_) => panic!("Error while reading data"),
            };
        });

        // added 0.1 ms delay to account for clear interrutp irq on radio 
        // add a blocking delay here with GPT2 or some other timer resource

        // clear the rxInt flag
        cx.local.rxInt.clear_triggered();
    }
}

