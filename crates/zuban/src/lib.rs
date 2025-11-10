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

use std::env;
use std::process::{Command, exit};

pub fn execute_zuban_executable_with_subcommand(mut subcommand: &str) {
    // Collect args except the binary name
    let mut args: Vec<String> = env::args().skip(1).collect();

    let mut zuban_path = env::current_exe().expect("failed to get current exe path");
    // replace "zubanls"/"zmypy" with "zuban"
    zuban_path.set_file_name("zuban");

    if args.len() == 1 && (args[0] == "--version" || args[0] == "-V") {
        args.clear();
        subcommand = "--version"
    }

    // Run "./zuban server <args...>"
    let status = Command::new(zuban_path)
        .arg(subcommand)
        .args(args)
        .status()
        .expect("failed to execute zuban");
    exit(status.code().unwrap_or(1));
}
