#![no_main]
#![no_std]
#![feature(default_alloc_error_handler)]

use nucleo::hal::prelude::*;
use nucleo::led::Led;
use nucleo_h7xx as nucleo;

use alloc_cortex_m::*;

use wasmi::*;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, h7_blinky!");

    let start = cortex_m_rt::heap_start() as usize;
    let size = 1048576;
    unsafe { ALLOCATOR.init(start, size) }

    defmt::info!("Heap is ready");

    // - board setup ----------------------------------------------------------

    let board = nucleo::board::Board::take().unwrap();

    let dp = nucleo::pac::Peripherals::take().unwrap();

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

    let mut user_leds = nucleo::led::UserLeds::new(pins.user_leds);

    // - main loop ------------------------------------------------------------

    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 1, 127, 1, 127, 3, 2, 1, 0, 7, 9, 1, 5, 99, 111,
        108, 111, 114, 0, 0, 10, 9, 1, 7, 0, 32, 0, 65, 3, 111, 11,
    ];

    let one_second = ccdr.clocks.sys_ck().0;

    let module = wasmi::Module::from_buffer(&wasm_binary).unwrap_or_else(|_| loop {
        user_leds.ld3.on();
        user_leds.ld3.off();
        cortex_m::asm::delay(one_second);
    });

    defmt::info!("Module is ready.");

    // Instantiate a module with empty imports and
    // assert that there is no `start` function.
    let instance: ModuleRef = ModuleInstance::new(&module, &ImportsBuilder::default())
        .unwrap_or_else(|_| loop {
            user_leds.ld2.on();
            user_leds.ld2.off();
            cortex_m::asm::delay(one_second);
        })
        .assert_no_start();

    defmt::info!("Instance is ready.");

    let mut n: usize = 0;
    loop {
        let val = instance
            .invoke_export("color", &[RuntimeValue::I32(n as i32)], &mut NopExternals)
            .unwrap_or_else(|_| loop {
                user_leds.ld3.on();
                user_leds.ld3.off();
                cortex_m::asm::delay(one_second);
            })
            .unwrap();

        if let RuntimeValue::I32(v) = val {
            match v {
                0 => user_leds.ld1.on(),
                1 => user_leds.ld2.on(),
                2 => user_leds.ld3.on(),
                _ => {
                    user_leds.ld1.on();
                    user_leds.ld2.on();
                    user_leds.ld3.on();
                }
            }
            cortex_m::asm::delay(one_second);
        }

        user_leds.ld1.off();
        user_leds.ld2.off();
        user_leds.ld3.off();

        defmt::info!("loop: {:?}", n + 1);
        n += 1;
    }
    h7_blinky::exit()
}
