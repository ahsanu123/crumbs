## Investigating esp-alloc different patch error 
 
That panic tells you an allocation failed—not why it failed. The allocator-source mismatch is something you discover by following the allocation path and checking Cargo’s dependency graph.

In fact, your current dependency graph contains two  esp-alloc  packages with the same version but different sources. I checked using offline Cargo commands only; no ESP connection or hardware tests.

1. Read the backtrace from the failure toward its caller

Your trace gives this chain:
```
 Wi-Fi initialization
   → ic_create_wifi_task
   → pp_create_task
   → Box<..., esp_alloc::allocators::InternalMemory>::new_uninit_in
   → handle_alloc_error
   → panic
```
The important clue is  InternalMemory , not  alloc.rs:573 . That  Box  explicitly uses an ESP allocator. The standard-library panic handler is just the final reporting point after allocation fails; it cannot distinguish an exhausted heap from an uninitialized allocator.

Also,  `.expect("fail to create new wifi")`  cannot catch this: the allocation panics inside initialization before  `wifi::new()` returns a  Result .

2. Check which allocator each dependency uses
```
Start with:

 cargo tree --offline --locked -d
 cargo tree --offline --locked -i esp-alloc
```
 -d  shows duplicate packages;  -i  shows who depends on a package. In your checkout, the second command reports an ambiguity and lists these two identities:
```
 registry+https://github.com/rust-lang/crates.io-index#esp-alloc@0.9.0
 git+https://github.com/esp-rs/esp-hal?rev=c96f07bcd2f5a2c617a2ac538471731a76ab3e02#esp-alloc@0.9.0
```
Pass each quoted identity to  cargo tree --offline --locked -i ...  to trace it. Your actual trees show:
```
 esp-alloc 0.9.0 [crates.io]
 └── tmd
 
 esp-alloc 0.9.0 [Git]
 └── esp-radio
     └── tmd
```
Same name and version does not mean the same crate instance. Cargo also distinguishes the source.  Cargo.lock  confirms the separate packages at lines 1998–2027.

3. Connect the dependency split to the failure

Your application calls:
```
 esp_alloc::heap_allocator!(...);
```
Those calls initialize the allocator package used directly by  tmd : the crates.io copy. Wi-Fi uses  InternalMemory  from the Git copy.

For an allocator backed by crate-local static state, those copies have separate heap state. Initializing one does not initialize the other. Thus Wi-Fi can fail to allocate even though your application registered plenty of memory with its own allocator.

To establish this from source, follow  InternalMemory ’s allocation implementation to its heap/static, then follow  heap_allocator!  to the heap it initializes. Check the package source as well as the filename when navigating in your IDE.

4. Distinguish this from ordinary out-of-memory

If the dependency graph has only one allocator instance, investigate initialization order, available internal memory, fragmentation/largest available block, and requested allocation size. Free memory in another region is not necessarily usable by  InternalMemory .

Duplicate dependencies alone are not proof of a bug; duplicates owning supposedly shared global state are the warning sign here. After aligning dependencies, repeat the tree inspection to verify both application initialization and Wi-Fi use the same allocator package. That is a stronger check than merely making their version strings match.

