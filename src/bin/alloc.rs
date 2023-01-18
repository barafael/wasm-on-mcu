#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use wasmi_m4 as _; // global logger + panicking-behavior + memory layout

extern crate alloc;

use core::alloc::Layout;
use embedded_alloc::Heap;

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("hello");
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
    }

    defmt::println!("pushing");
    let _xs = alloc::vec![1];

    // Do not remove - pulls in the panic handler
    #[allow(unreachable_code)]
    wasmi_m4::exit()
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {
        continue;
    }
}
