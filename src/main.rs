#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

// pick a panicking behavior
use panic_halt as _;

use cortex_m_rt::entry;

use alloc_cortex_m::*;

use wasmi::*;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn main() -> ! {
    let wasm_binary = [0, 10, 20, 3, 2];
    let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");

    // Instantiate a module with empty imports and
    // assert that there is no `start` function.
    let instance: ModuleRef = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    // Finally, invoke the exported function "test" with no parameters
    // and empty external function executor.
    assert_eq!(
        instance
            .invoke_export("test", &[], &mut NopExternals,)
            .expect("failed to execute export"),
        Some(RuntimeValue::I32(1337)),
    );

    loop {
        cortex_m::asm::wfi();
    }
}
