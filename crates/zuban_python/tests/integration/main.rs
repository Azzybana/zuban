#[cfg(all(feature = "mimalloc", feature = "jemalloc"))]
compile_error!("Cannot enable both mimalloc and jemalloc features at the same time");

#[cfg(all(not(target_env = "gnu"), feature = "jemalloc"))]
compile_error!("Cannot enable jemalloc without a gnu toolchain at this time");

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(feature = "jemalloc")]
use jemallocator::Jemalloc;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod signature_tests;
