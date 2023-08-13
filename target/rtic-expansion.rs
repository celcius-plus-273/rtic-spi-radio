#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
    r" Always include the device crate which contains the vector table"] use
    bsp as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ;
    #[doc =
    r" Holds the maximum priority level for use by async HAL drivers."]
    #[no_mangle] static RTIC_ASYNC_MAX_LOGICAL_PRIO : u8 = 1 << bsp ::
    NVIC_PRIO_BITS ; use super :: * ; const SYST_MONO_FACTOR : u32 = 10 ;
    const DELAY_MS : u32 = SYST_MONO_FACTOR * 1000 ; type Led = gpio :: Output
    < P8 > ; #[doc = r" User code end"] #[doc = r"Shared resources"] struct
    Shared { counter : u32, } #[doc = r"Local resources"] struct Local
    { spi : board :: Lpspi4, led : Led, } #[doc = r" Execution context"]
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
        let board :: Resources { pins, mut gpio2, usb, lpspi4, .. } = board ::
        t40(cx.device) ; bsp :: LoggingFrontend ::
        default_log().register_usb(usb) ; let systick_token = rtic_monotonics
        :: create_systick_token! () ; Systick ::
        start(cx.core.SYST, 36_000_000, systick_token) ; let led =
        gpio2.output(pins.p8) ; let counter = 0 ; let spi = board ::
        lpspi(lpspi4, board :: LpspiPins
        { pcs0 : pins.p10, sck : pins.p13, sdo : pins.p11, sdi : pins.p12, },
        1_000_000) ; toggle :: spawn().unwrap() ; talk_to_radio ::
        spawn().unwrap() ; (Shared { counter }, Local { spi, led })
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
    } impl < 'a > __rtic_internal_talk_to_radioLocalResources < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_talk_to_radioLocalResources
            {
                spi : & mut *
                (& mut *
                __rtic_internal_local_resource_spi.get_mut()).as_mut_ptr(),
                __rtic_internal_marker : :: core :: marker :: PhantomData,
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
                rtic :: export :: pend(bsp :: interrupt :: GPT1) ; Ok(())
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
    #[doc = "Local resources `talk_to_radio` has access to"] pub struct
    __rtic_internal_talk_to_radioLocalResources < 'a >
    {
        #[allow(missing_docs)] pub spi : & 'a mut board :: Lpspi4,
        #[doc(hidden)] pub __rtic_internal_marker : :: core :: marker ::
        PhantomData < & 'a () >,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_talk_to_radio_Context < 'a >
    {
        #[doc(hidden)] __rtic_internal_p : :: core :: marker :: PhantomData <
        & 'a () >, #[doc = r" Local Resources this task has access to"] pub
        local : talk_to_radio :: LocalResources < 'a >,
    } impl < 'a > __rtic_internal_talk_to_radio_Context < 'a >
    {
        #[inline(always)] #[allow(missing_docs)] pub unsafe fn new() -> Self
        {
            __rtic_internal_talk_to_radio_Context
            {
                __rtic_internal_p : :: core :: marker :: PhantomData, local :
                talk_to_radio :: LocalResources :: new(),
            }
        }
    } #[doc = r" Spawns the task directly"] #[allow(non_snake_case)]
    #[doc(hidden)] pub fn __rtic_internal_talk_to_radio_spawn() -> Result <
    (), () >
    {
        #[inline(always)] fn tait_hack() -> __rtic_internal_talk_to_radio_F
        { talk_to_radio(unsafe { talk_to_radio :: Context :: new() }) } unsafe
        {
            if __rtic_internal_talk_to_radio_EXEC.try_allocate()
            {
                let f = tait_hack() ;
                __rtic_internal_talk_to_radio_EXEC.spawn(f) ; rtic :: export
                :: pend(bsp :: interrupt :: GPT1) ; Ok(())
            } else { Err(()) }
        }
    } #[allow(non_snake_case)] #[doc = "Software task"] pub mod talk_to_radio
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_talk_to_radioLocalResources as LocalResources ;
        #[doc(inline)] pub use super :: __rtic_internal_talk_to_radio_Context
        as Context ; #[doc(inline)] pub use super ::
        __rtic_internal_talk_to_radio_spawn as spawn ;
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
    } #[allow(non_snake_case)] async fn talk_to_radio < 'a >
    (cx : talk_to_radio :: Context < 'a >)
    {
        use rtic :: Mutex as _ ; use rtic :: mutex :: prelude :: * ; let spi =
        cx.local.spi ; loop
        {
            Systick :: delay(DELAY_MS.millis()).await ; log :: info!
            ("asking for radio's version...") ; let reg : u8 = 0x42 ; let mut
            buffer = [reg & 0x7f, 0] ; let result =
            spi.transfer(& mut buffer).unwrap() ; log :: info!
            ("radio's version is: {:#04x}. Expected version: 0x12", result [1]
            as u8) ;
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
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_local_resource_spi : rtic :: RacyCell < core :: mem ::
    MaybeUninit < board :: Lpspi4 >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic2"] static
    __rtic_internal_local_resource_led : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Led >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()) ;
    #[allow(non_camel_case_types)] type __rtic_internal_toggle_F = impl core
    :: future :: Future ; #[allow(non_upper_case_globals)] static
    __rtic_internal_toggle_EXEC : rtic :: export :: executor ::
    AsyncTaskExecutor < __rtic_internal_toggle_F > = rtic :: export ::
    executor :: AsyncTaskExecutor :: new() ; #[allow(non_camel_case_types)]
    type __rtic_internal_talk_to_radio_F = impl core :: future :: Future ;
    #[allow(non_upper_case_globals)] static __rtic_internal_talk_to_radio_EXEC
    : rtic :: export :: executor :: AsyncTaskExecutor <
    __rtic_internal_talk_to_radio_F > = rtic :: export :: executor ::
    AsyncTaskExecutor :: new() ; #[allow(non_snake_case)]
    #[doc = "Interrupt handler to dispatch async tasks at priority 1"]
    #[no_mangle] unsafe fn GPT1()
    {
        #[doc = r" The priority of this interrupt handler"] const PRIORITY :
        u8 = 1u8 ; rtic :: export ::
        run(PRIORITY, ||
        {
            __rtic_internal_talk_to_radio_EXEC.poll(||
            {
                __rtic_internal_talk_to_radio_EXEC.set_pending() ; rtic ::
                export :: pend(bsp :: interrupt :: GPT1) ;
            }) ;
            __rtic_internal_toggle_EXEC.poll(||
            {
                __rtic_internal_toggle_EXEC.set_pending() ; rtic :: export ::
                pend(bsp :: interrupt :: GPT1) ;
            }) ;
        }) ;
    } #[doc(hidden)] #[no_mangle] unsafe extern "C" fn main() ->!
    {
        rtic :: export :: assert_send :: < u32 > () ; rtic :: export ::
        interrupt :: disable() ; let mut core : rtic :: export :: Peripherals
        = rtic :: export :: Peripherals :: steal().into() ; let _ =
        you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml ::
        interrupt :: GPT1 ; const _ : () = if(1 << bsp :: NVIC_PRIO_BITS) <
        1u8 as usize
        {
            :: core :: panic!
            ("Maximum priority used by interrupt vector 'GPT1' is more than supported by hardware")
            ;
        } ;
        core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: GPT1, rtic :: export ::
        cortex_logical2hw(1u8, bsp :: NVIC_PRIO_BITS),) ; rtic :: export ::
        NVIC ::
        unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
        :: interrupt :: GPT1) ; #[inline(never)] fn __rtic_init_resources < F
        > (f : F) where F : FnOnce() { f() ; }
        __rtic_init_resources(||
        {
            let(shared_resources, local_resources) =
            init(init :: Context :: new(core.into())) ;
            __rtic_internal_shared_resource_counter.get_mut().write(core ::
            mem :: MaybeUninit :: new(shared_resources.counter)) ;
            __rtic_internal_local_resource_spi.get_mut().write(core :: mem ::
            MaybeUninit :: new(local_resources.spi)) ;
            __rtic_internal_local_resource_led.get_mut().write(core :: mem ::
            MaybeUninit :: new(local_resources.led)) ; rtic :: export ::
            interrupt :: enable() ;
        }) ; idle(idle :: Context :: new())
    }
}