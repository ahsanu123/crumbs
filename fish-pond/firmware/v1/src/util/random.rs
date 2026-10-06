// export const random = (min: number, max: number) => {
//   return min + Math.random() * (max - min)
// }
//
// export const getRandom = (arr: number[]) => {
//   const index = Math.floor(random(0, arr.length))
//   return arr[index]
// }

use core::cell::RefCell;
use static_cell::StaticCell;

#[cfg(feature = "std")]
use std::sync::Mutex as PlatformMutex;

#[cfg(not(feature = "std"))]
use critical_section::Mutex as PlatformMutex;

// static RNG: PlatformMutex<RefCell<u8>> = PlatformMutex::new(RefCell::new(8u8));
// // ---- Option A: std Target ----
//     // Automatically fetches a high-quality seed from the OS
//     let mut std_rng = StdRng::from_rng(&mut rand::rng()).unwrap();
//     let choice_a = perform_game_logic(&mut std_rng);
//
//     // ---- Option B: no_std Target (Deterministic / Manual Seed) ----
//     // Perfect for embedded systems where you provide a hardcoded seed,
//     // a reading from an ADC pin, or a device serial number.
//     let manual_seed: u64 = 123456789;
//     let mut nostd_rng = SmallRng::seed_from_u64(manual_seed);
//     let choice_b = perform_game_logic(&mut nostd_rng);

pub fn random(min: f32, max: f32) {}

pub fn get_random() {}
