#![no_main]
#![no_std]
#![feature(default_alloc_error_handler)]

use stm32f4xx_hal::{prelude::*, rcc::RccExt};
use wasmi_m4 as _; // global logger + panicking-behavior + memory layout

use alloc_cortex_m::CortexMHeap;
use embedded_hal::blocking::delay::DelayMs;

use wasmi::{Caller, Config, Engine, Extern, Func, Linker, Module, StackLimits, Store};

type HostState = u32;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello Wasmi :)");

    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024 * 16;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
    }

    defmt::info!("Heap is ready.");

    // - board setup ----------------------------------------------------------
    let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48u32.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);

    defmt::info!("Board is ready.");

    // - module load -----------------------------------------------------------
    let mut config = Config::default();
    config.set_stack_limits(StackLimits::new(256, 512, 128).unwrap());

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

        delay.delay_ms(1000_u16);

        n += 1;
    }
    // Do not remove - pulls in the panic handler
    #[allow(unreachable_code)]
    wasmi_m4::exit()
}
