#![no_main]
#![no_std]

use embedded_alloc::LlffHeap as Heap;
// Two-Level Segregated Fit Heap allocator (feature = "tlsf")
// use embedded_alloc::TlsfHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn init_alloc() {
    unsafe {
        embedded_alloc::init!(HEAP, 1024);
    }
}

// defmt-test 0.3.0 has the limitation that this `#[tests]` attribute can only be used
// once within a crate. the module can be in any file but there can only be at most
// one `#[tests]` module in this library crate
// #[cfg(test)]
// #[defmt_test::tests]
// mod unit_tests {
//     use defmt::assert;
//
//     #[test]
//     fn it_works() {
//         assert!(true)
//     }
// }
