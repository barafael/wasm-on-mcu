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
        0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 127, 3, 2, 1, 0, 7, 8, 1, 4, 116, 101, 115,
        116, 0, 0, 10, 7, 1, 5, 0, 65, 185, 10, 11,
    ];

    let one_second = ccdr.clocks.sys_ck().0;

    let module = wasmi::Module::from_buffer(&wasm_binary).unwrap_or_else(|_| loop {
        user_leds.ld3.on();
        user_leds.ld3.off();
        cortex_m::asm::delay(one_second);
    });

    // Instantiate a module with empty imports and
    // assert that there is no `start` function.
    let instance: ModuleRef = ModuleInstance::new(&module, &ImportsBuilder::default())
        .unwrap_or_else(|_| loop {
            user_leds.ld2.on();
            user_leds.ld2.off();
            cortex_m::asm::delay(one_second);
        })
        .assert_no_start();

    // Finally, invoke the exported function "test" with no parameters
    // and empty external function executor.
    assert_eq!(
        instance
            .invoke_export("test", &[], &mut NopExternals,)
            .unwrap_or_else(|_| {
                loop {
                    user_leds.ld3.on();
                    user_leds.ld3.off();
                    cortex_m::asm::delay(one_second);
                }
            }),
        Some(RuntimeValue::I32(1337)),
    );

    for n in 0..10 {
        user_leds.ld3.off();
        user_leds.ld1.on();
        cortex_m::asm::delay(one_second);

        user_leds.ld1.off();
        user_leds.ld2.on();
        cortex_m::asm::delay(one_second);

        user_leds.ld2.off();
        user_leds.ld3.on();
        cortex_m::asm::delay(one_second);

        defmt::info!("loop: {:?} of 10", n + 1);
    }

    h7_blinky::exit()
}
