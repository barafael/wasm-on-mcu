#![no_main]
#![no_std]
#![feature(default_alloc_error_handler)]
// For `T::addr()` of heap start
#![feature(strict_provenance)]

use alloc_cortex_m::CortexMHeap;
use embedded_hal::blocking::delay::DelayMs;
use nucleo_h7xx::{
    hal::{
        prelude::{_stm32h7xx_hal_delay_DelayExt, _stm32h7xx_hal_gpio_GpioExt},
        pwr::PwrExt,
        rcc::RccExt,
    },
    led::Led,
    led::UserLeds,
    Board,
};

use wasmi::{Caller, Config, Engine, Extern, Func, Linker, Module, Store};

type HostState = u32;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, wasm-on-mcu!");

    let start = cortex_m_rt::heap_start().addr();
    let size = 0x0010_0000;
    unsafe { ALLOCATOR.init(start, size) }

    defmt::info!("Heap is ready");

    // - board setup ----------------------------------------------------------

    let board = Board::take().unwrap();

    let dp = nucleo_h7xx::pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let ccdr = board.freeze_clocks(dp.PWR.constrain(), dp.RCC.constrain(), &dp.SYSCFG);

    let pins = board.split_gpios(
        dp.GPIOA.split(ccdr.peripheral.GPIOA),
        dp.GPIOB.split(ccdr.peripheral.GPIOB),
        dp.GPIOC.split(ccdr.peripheral.GPIOC),
        dp.GPIOD.split(ccdr.peripheral.GPIOD),
        dp.GPIOE.split(ccdr.peripheral.GPIOE),
        dp.GPIOF.split(ccdr.peripheral.GPIOF),
        dp.GPIOG.split(ccdr.peripheral.GPIOG),
    );

    let mut user_leds = UserLeds::new(pins.user_leds);

    let mut delay = cp.SYST.delay(ccdr.clocks);

    // - module load -----------------------------------------------------------
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    // TODO adapt config for lower stack usage (and then try running on Cortex-M4 target!)
    let config = Config::default();

    let engine = Engine::new(&config);

    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 1, 127, 1, 127, 3, 2, 1, 0, 7, 9, 1, 5, 99, 111,
        108, 111, 114, 0, 0, 10, 9, 1, 7, 0, 32, 0, 65, 3, 111, 11,
    ];

    let module = Module::new(&engine, &wasm_binary[..]).unwrap();

    defmt::info!("Module is ready.");

    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    let mut store = Store::new(&engine, 42);
    let host_hello = Func::wrap(&mut store, |caller: Caller<'_, HostState>, param: i32| {
        defmt::println!("Got {} from WebAssembly", param);
        defmt::println!("My host state is: {}", caller.host_data());
    });

    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let mut linker = <Linker<HostState>>::new();
    // Instantiation of a Wasm module requires defning its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    linker.define("host", "color", host_hello).unwrap();
    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    let hello = instance
        .get_export(&store, "color")
        .and_then(Extern::into_func)
        .expect("could not find function \"color\"")
        .typed::<i32, i32, _>(&mut store)
        .unwrap();

    // - main loop ------------------------------------------------------------

    // And finally we can call the wasm!
    let mut n = 0;
    loop {
        let num = hello.call(&mut store, n).unwrap();

        defmt::info!("num: {}", num);

        match num {
            0 => user_leds.ld1.on(),
            1 => user_leds.ld2.on(),
            2 => user_leds.ld3.on(),
            _ => {
                user_leds.ld1.on();
                user_leds.ld2.on();
                user_leds.ld3.on();
            }
        }
        delay.delay_ms(1000_u16);

        user_leds.ld1.off();
        user_leds.ld2.off();
        user_leds.ld3.off();

        n += 1;
    }
    // Do not remove - pulls in the panic handler
    #[allow(unreachable_code)]
    wasm_on_mcu::exit()
}
