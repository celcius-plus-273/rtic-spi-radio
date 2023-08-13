#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
    r" Always include the device crate which contains the vector table"] use
    bsp as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ;
    #[doc =
    r" Holds the maximum priority level for use by async HAL drivers."]
    #[no_mangle] static RTIC_ASYNC_MAX_LOGICAL_PRIO : u8 = 0u8 ; use super ::
    * ; const SYST_MONO_FACTOR : u32 = 10 ; const DELAY_MS : u32 =
    SYST_MONO_FACTOR * 2000 ; const LONG_DELAY : u32 = SYST_MONO_FACTOR *
    10_000 ; const FREQUENCY : i64 = 915 ; const GPT1_FREQUENCY : u32 = 1_000
    ; const GPT1_CLOCK_SOURCE : ClockSource = ClockSource ::
    HighFrequencyReferenceClock ; const GPT1_DIVIDER : u32 = board ::
    PERCLK_FREQUENCY / GPT1_FREQUENCY ; type Led = gpio :: Output < P7 > ;
    type Delay = Blocking < Gpt1, GPT1_FREQUENCY > ; type Interrupt = gpio ::
    Input < P1 > ; type Lora_Radio = sx127x_lora :: LoRa < board :: Lpspi4,
    gpio :: Output < P8 >, gpio :: Output < P9 >, Delay > ;
    #[doc = r" User code end"] #[doc = r"Shared resources"] struct Shared
    { counter : u32, lora : Lora_Radio, } #[doc = r"Local resources"] struct
    Local { led : Led, rxInt : Interrupt, } #[doc = r" Execution context"]
    #[allow(non_snake_case)] #[allow(non_camel_case_types)] pub struct
    __rtic_internal_init_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Core peripherals"] pub core : rtic :: export ::
        Peripherals, #[doc = r" Device peripherals (PAC)"] pub device : bsp ::
        Peripherals, #[doc = r" Critical section token for init"] pub cs :
        rtic :: export :: CriticalSection < 'a >,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn
        new(core : rtic :: export :: Peripherals,) -> Self
        {
            __rtic_internal_init_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, device :
                bsp :: Peripherals :: steal(), cs : rtic :: export ::
                CriticalSection :: new(), core,
            }
        }
    } #[allow(non_snake_case)] #[doc = "Initialization function"] pub mod init
    {
        #[doc(inline)] pub use super :: __rtic_internal_init_Context as
        Context ;
    } #[inline(always)] #[allow(non_snake_case)] fn init(cx : init :: Context)
    -> (Shared, Local)
    {
        let board :: Resources
        { pins, mut gpio1, mut gpio2, usb, lpspi4, mut gpt1, .. } = board ::
        t40(cx.device) ; bsp :: LoggingFrontend ::
        default_log().register_usb(usb) ; let systick_token = rtic_monotonics
        :: create_systick_token! () ; Systick ::
        start(cx.core.SYST, 36_000_000, systick_token) ; gpt1.disable() ;
        gpt1.set_divider(GPT1_DIVIDER) ;
        gpt1.set_clock_source(GPT1_CLOCK_SOURCE) ; let mut delay = Blocking ::
        < _, GPT1_FREQUENCY > :: from_gpt(gpt1) ; let rxInt =
        gpio1.input(pins.p1) ;
        gpio1.set_interrupt(& rxInt, Some(Trigger :: RisingEdge)) ; let led =
        gpio2.output(pins.p7) ; let counter = 0 ; let mut spi = board ::
        lpspi(lpspi4, board :: LpspiPins
        { pcs0 : pins.p10, sck : pins.p13, sdo : pins.p11, sdi : pins.p12, },
        1_000_000) ; let fake_cs = gpio2.output(pins.p8) ; let reset =
        gpio2.output(pins.p9) ; let mut lora = match sx127x_lora :: LoRa ::
        new(spi, fake_cs, reset, FREQUENCY, delay)
        {
            Ok(sx127x) => sx127x, Err(error) => match error
            {
                sx127x_lora :: Error :: VersionMismatch(version) => panic!
                ("Version Mismatch Error. Version{:?}", version), sx127x_lora
                :: Error :: CS(_) => panic! ("Chip select issue"), sx127x_lora
                :: Error :: Reset(_) => panic! ("Reset issue"), sx127x_lora ::
                Error :: SPI(_) => panic! ("SPI problem"), sx127x_lora ::
                Error :: Transmitting => panic!
                ("Error during spi transmission"), sx127x_lora :: Error ::
                Uninformative => panic! ("Uninformative error RIP"),
            }
        } ; toggle :: spawn().unwrap() ; init_radio :: spawn().unwrap() ;
        (Shared { counter, lora }, Local { led, rxInt })
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_idle_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >,
    } impl < 'a > __rtic_internal_idle_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_idle_Context
            { __rtic_internal_p : :: core :: marker :: PhantomData, }
        }
    } #[allow(non_snake_case)] #[doc = "Idle loop"] pub mod idle
    {
        #[doc(inline)] pub use super :: __rtic_internal_idle_Context as
        Context ;
    } #[allow(non_snake_case)] fn idle(_ : idle :: Context) ->!
    {
        use rtic :: Mutex as _ ; use rtic :: mutex :: prelude :: * ; loop
        { cortex_m :: asm :: wfi() ; }
    } #[allow(non_snake_case)] #[no_mangle] unsafe fn GPIO1_COMBINED_0_15()
    {
        const PRIORITY : u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, || { receive_data(receive_data :: Context :: new()) }) ;
    } impl < 'a > __rtic_internal_receive_dataLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_receive_dataLocalResources
            {
                rxInt : & mut *
                (& mut *
                __rtic_internal_local_resource_rxInt.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_receive_dataSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_receive_dataSharedResources
            {
                lora : shared_resources :: lora_that_needs_to_be_locked ::
                new(), __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `receive_data` has access to"] pub struct
    __rtic_internal_receive_dataLocalResources < 'a >
    {
        #[allow(missing_docs)] pub rxInt : & 'a mut Interrupt, #[doc(hidden)]
        pub __rtic_internal_marker : :: core :: marker :: PhantomData < & 'a
        () >,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `receive_data` has access to"] pub struct
    __rtic_internal_receive_dataSharedResources < 'a >
    {
        #[allow(missing_docs)] pub lora : shared_resources ::
        lora_that_needs_to_be_locked < 'a >, #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_receive_data_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Local Resources this task has access to"] pub
        local : receive_data :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        receive_data :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_receive_data_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_receive_data_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                receive_data :: LocalResources :: new(), shared : receive_data
                :: SharedResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = "Hardware task"] pub mod receive_data
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_receive_dataLocalResources as LocalResources ;
        #[doc(inline)] pub use super ::
        __rtic_internal_receive_dataSharedResources as SharedResources ;
        #[doc(inline)] pub use super :: __rtic_internal_receive_data_Context
        as Context ;
    } #[allow(non_snake_case)] fn receive_data(cx : receive_data :: Context)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex :: prelude :: * ; let mut
        lora = cx.shared.lora ;
        #[doc = " Interrupt is being triggered twice... not sure why"]
        #[doc = " maybe just add a boolean check?"]
        #[doc =
        " When adding a slight delaya it doesn\'t get triggered twice anymore..."]
        #[doc =
        " I think the radio is taking some time itself to clear the RxDone irq"]
        #[doc = " Added a slight delay to compensate for that"] log :: info!
        ("Interrupt triggered!") ;
        lora.lock(| lora |
        {
            match lora.read_packet()
            {
                Ok(buffer) => for c in buffer
                {
                    if c == 0x0 { break } ; log :: info! ("got: {}", c as char)
                    ;
                }, Err(_) => panic! ("Error while reading data"),
            } ;
        }) ; cx.local.rxInt.clear_triggered() ;
    } impl < 'a > __rtic_internal_toggleLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_toggleLocalResources
            {
                led : & mut *
                (& mut *
                __rtic_internal_local_resource_led.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_toggleSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_toggleSharedResources
            {
                counter : shared_resources :: counter_that_needs_to_be_locked
                :: new(), __rtic_internal_marker : core :: marker ::
                PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_init_radioSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_init_radioSharedResources
            {
                lora : shared_resources :: lora_that_needs_to_be_locked ::
                new(), __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } impl < 'a > __rtic_internal_senderSharedResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_senderSharedResources
            {
                lora : shared_resources :: lora_that_needs_to_be_locked ::
                new(), __rtic_internal_marker : core :: marker :: PhantomData,
            }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Local resources `toggle` has access to"] pub struct
    __rtic_internal_toggleLocalResources < 'a >
    {
        #[allow(missing_docs)] pub led : & 'a mut Led, #[doc(hidden)] pub
        __rtic_internal_marker : :: core :: marker :: PhantomData < & 'a () >,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `toggle` has access to"] pub struct
    __rtic_internal_toggleSharedResources < 'a >
    {
        #[allow(missing_docs)] pub counter : shared_resources ::
        counter_that_needs_to_be_locked < 'a >, #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_toggle_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Local Resources this task has access to"] pub
        local : toggle :: LocalResources < 'a >,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        toggle :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_toggle_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_toggle_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                toggle :: LocalResources :: new(), shared : toggle ::
                SharedResources :: new(),
            }
        }
    } #[doc = r" Spawns the task directly"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_toggle_spawn() -> Result < (), () >
    {
        #[inline(always)] fn tait_hack() -> __rtic_internal_toggle_F
        { toggle(unsafe { toggle :: Context :: new() }) } unsafe
        {
            if __rtic_internal_toggle_EXEC.try_allocate()
            {
                let f = tait_hack() ; __rtic_internal_toggle_EXEC.spawn(f) ;
                rtic :: export :: pend(bsp :: interrupt :: GPT2) ; Ok(())
            } else { Err(()) }
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod toggle
    {
        #[doc(inline)] pub use super :: __rtic_internal_toggleLocalResources
        as LocalResources ; #[doc(inline)] pub use super ::
        __rtic_internal_toggleSharedResources as SharedResources ;
        #[doc(inline)] pub use super :: __rtic_internal_toggle_Context as
        Context ; #[doc(inline)] pub use super :: __rtic_internal_toggle_spawn
        as spawn ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `init_radio` has access to"] pub struct
    __rtic_internal_init_radioSharedResources < 'a >
    {
        #[allow(missing_docs)] pub lora : shared_resources ::
        lora_that_needs_to_be_locked < 'a >, #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_init_radio_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Shared Resources this task has access to"] pub
        shared : init_radio :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_init_radio_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_init_radio_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, shared :
                init_radio :: SharedResources :: new(),
            }
        }
    } #[doc = r" Spawns the task directly"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_init_radio_spawn() -> Result < (),
    () >
    {
        #[inline(always)] fn tait_hack() -> __rtic_internal_init_radio_F
        { init_radio(unsafe { init_radio :: Context :: new() }) } unsafe
        {
            if __rtic_internal_init_radio_EXEC.try_allocate()
            {
                let f = tait_hack() ; __rtic_internal_init_radio_EXEC.spawn(f)
                ; rtic :: export :: pend(bsp :: interrupt :: GPT2) ; Ok(())
            } else { Err(()) }
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod init_radio
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_init_radioSharedResources as SharedResources ;
        #[doc(inline)] pub use super :: __rtic_internal_init_radio_Context as
        Context ; #[doc(inline)] pub use super ::
        __rtic_internal_init_radio_spawn as spawn ;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = "Shared resources `sender` has access to"] pub struct
    __rtic_internal_senderSharedResources < 'a >
    {
        #[allow(missing_docs)] pub lora : shared_resources ::
        lora_that_needs_to_be_locked < 'a >, #[doc(hidden)] pub
        __rtic_internal_marker : core :: marker :: PhantomData < & 'a () >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_sender_Context <
    'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Shared Resources this task has access to"] pub
        shared : sender :: SharedResources < 'a >,
    } impl < 'a > __rtic_internal_sender_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_sender_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, shared :
                sender :: SharedResources :: new(),
            }
        }
    } #[doc = r" Spawns the task directly"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_sender_spawn() -> Result < (), () >
    {
        #[inline(always)] fn tait_hack() -> __rtic_internal_sender_F
        { sender(unsafe { sender :: Context :: new() }) } unsafe
        {
            if __rtic_internal_sender_EXEC.try_allocate()
            {
                let f = tait_hack() ; __rtic_internal_sender_EXEC.spawn(f) ;
                rtic :: export :: pend(bsp :: interrupt :: GPT2) ; Ok(())
            } else { Err(()) }
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod sender
    {
        #[doc(inline)] pub use super :: __rtic_internal_senderSharedResources
        as SharedResources ; #[doc(inline)] pub use super ::
        __rtic_internal_sender_Context as Context ; #[doc(inline)] pub use
        super :: __rtic_internal_sender_spawn as spawn ;
    } #[allow(non_snake_case)] async fn toggle < 'a >
    (cx : toggle :: Context < 'a >)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex :: prelude :: * ; let mut
        counter = cx.shared.counter ; loop
        {
            counter.lock(| counter |
            { * counter += 1 ; log :: info! ("blink # {}!", * counter) ; }) ;
            cx.local.led.toggle() ; Systick :: delay(DELAY_MS.millis()).await
            ;
        }
    } #[allow(non_snake_case)] async fn init_radio < 'a >
    (cx : init_radio :: Context < 'a >)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex :: prelude :: * ; let mut
        lora = cx.shared.lora ; Systick :: delay(DELAY_MS.millis()).await ;
        lora.lock(| lora |
        {
            log :: info!
            ("lora's version is: {:#04x}", lora.get_radio_version().unwrap())
            ; match lora.set_tx_power(17, 1)
            {
                Ok(_) => log :: info! ("Succesfully set TX power"), Err(_) =>
                panic! ("Error setting tx power"),
            } ; log :: info! ("Succesfully init LoRa!") ;
        }) ; sender :: spawn().unwrap() ;
    } #[allow(non_snake_case)] async fn sender < 'a >
    (cx : sender :: Context < 'a >)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex :: prelude :: * ; let mut
        lora = cx.shared.lora ; let message = "IT WORKS!!" ; let mut buffer =
        [0 ; 255] ; for(i, c) in message.chars().enumerate()
        { buffer [i] = c as u8 ; } loop
        {
            log :: info! ("Sending... message: {message}") ;
            lora.lock(| lora |
            {
                match lora.transmit_payload_busy(buffer, message.len())
                {
                    Ok(packet_size) => log :: info!
                    ("Sent packet with size: {}", packet_size), Err(_) => panic!
                    ("Error sending packet x.x"),
                } ;
            }) ; Systick :: delay(LONG_DELAY.millis()).await ;
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
    __rtic_internal_shared_resource_counter : rtic :: RacyCell < core :: mem
    :: MaybeUninit < u32 >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; impl < 'a > rtic :: Mutex
    for shared_resources :: counter_that_needs_to_be_locked < 'a >
    {
        type T = u32 ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut u32) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_counter.get_mut() as *
                mut _, CEILING, bsp :: NVIC_PRIO_BITS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_shared_resource_lora : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Lora_Radio >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ; impl < 'a > rtic :: Mutex
    for shared_resources :: lora_that_needs_to_be_locked < 'a >
    {
        type T = Lora_Radio ; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut Lora_Radio) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8 ; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_lora.get_mut() as * mut
                _, CEILING, bsp :: NVIC_PRIO_BITS, f,)
            }
        }
    } mod shared_resources
    {
        #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        counter_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () >, }
        impl < 'a > counter_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                counter_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        lora_that_needs_to_be_locked < 'a >
        { __rtic_internal_p : :: core :: marker :: PhantomData < & 'a () >, }
        impl < 'a > lora_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new() -> Self
            {
                lora_that_needs_to_be_locked
                { __rtic_internal_p : :: core :: marker :: PhantomData }
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic2"] static
    __rtic_internal_local_resource_led : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Led >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic3"] static
    __rtic_internal_local_resource_rxInt : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Interrupt >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] type __rtic_internal_toggle_F = impl core
    :: future :: Future ; #[allow(non_upper_case_globals)] static
    __rtic_internal_toggle_EXEC : rtic :: export :: executor ::
    AsyncTaskExecutor < __rtic_internal_toggle_F > = rtic :: export ::
    executor :: AsyncTaskExecutor :: new() ; #[allow(non_camel_case_types)]
    type __rtic_internal_init_radio_F = impl core :: future :: Future ;
    #[allow(non_upper_case_globals)] static __rtic_internal_init_radio_EXEC :
    rtic :: export :: executor :: AsyncTaskExecutor <
    __rtic_internal_init_radio_F > = rtic :: export :: executor ::
    AsyncTaskExecutor :: new() ; #[allow(non_camel_case_types)] type
    __rtic_internal_sender_F = impl core :: future :: Future ;
    #[allow(non_upper_case_globals)] static __rtic_internal_sender_EXEC : rtic
    :: export :: executor :: AsyncTaskExecutor < __rtic_internal_sender_F > =
    rtic :: export :: executor :: AsyncTaskExecutor :: new() ;
    #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch async tasks at priority 1"]
    #[no_mangle] unsafe fn GPT2()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
        {
            __rtic_internal_init_radio_EXEC.poll(||
            {
                __rtic_internal_init_radio_EXEC.set_pending() ; rtic :: export
                :: pend(bsp :: interrupt :: GPT2) ;
            }) ;
            __rtic_internal_sender_EXEC.poll(||
            {
                __rtic_internal_sender_EXEC.set_pending() ; rtic :: export ::
                pend(bsp :: interrupt :: GPT2) ;
            }) ;
            __rtic_internal_toggle_EXEC.poll(||
            {
                __rtic_internal_toggle_EXEC.set_pending() ; rtic :: export ::
                pend(bsp :: interrupt :: GPT2) ;
            }) ;
        }) ;
    } #[doc(hidden)] #[no_mangle] unsafe extern "C" fn main() ->!
    {
        rtic :: export :: assert_send :: < u32 > () ; rtic :: export ::
        assert_send :: < Lora_Radio > () ; rtic :: export :: interrupt ::
        disable() ; let mut core : rtic :: export :: Peripherals = rtic ::
        export :: Peripherals :: steal().into() ; let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: GPT2 ; const _ : () = if(1 << bsp :: NVIC_PRIO_BITS) <
        1u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'GPT2' is more than supported by hardware")
            ;
        } ;
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: GPT2, rtic :: export ::
        cortex_logical2hw(1u8, bsp :: NVIC_PRIO_BITS),) ; rtic :: export ::
        NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: GPT2) ; const _ : () = if(1 << bsp :: NVIC_PRIO_BITS)
        < 1u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'GPIO1_COMBINED_0_15' is more than supported by hardware")
            ;
        } ;
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: GPIO1_COMBINED_0_15, rtic :: export ::
        cortex_logical2hw(1u8, bsp :: NVIC_PRIO_BITS),) ; rtic :: export ::
        NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: GPIO1_COMBINED_0_15) ; #[inline(never)] fn
        __rtic_init_resources < F > (f : F) where F : FnOnce() { f() ; }
        __rtic_init_resources(||
        {
            let(shared_resources, local_resources) =
            init(init :: Context :: new(core.into())) ;
            __rtic_internal_shared_resource_counter.get_mut().write(core ::
            mem :: MaybeUninit :: new(shared_resources.counter)) ;
            __rtic_internal_shared_resource_lora.get_mut().write(core :: mem
            :: MaybeUninit :: new(shared_resources.lora)) ;
            __rtic_internal_local_resource_led.get_mut().write(core :: mem ::
            MaybeUninit :: new(local_resources.led)) ;
            __rtic_internal_local_resource_rxInt.get_mut().write(core :: mem
            :: MaybeUninit :: new(local_resources.rxInt)) ; rtic :: export ::
            interrupt :: enable() ;
        }) ; idle(idle :: Context :: new())
    }
}