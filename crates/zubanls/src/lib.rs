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

mod capabilities;
mod notebooks;
mod notification_handlers;
mod panic_hooks;
mod request_handlers;
mod semantic_tokens;
mod server;

pub use crate::server::{
    GLOBAL_NOTIFY_EVENT_COUNTER, run_server, run_server_with_custom_connection,
};
