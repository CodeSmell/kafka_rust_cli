mod args;
//mod content;
mod file;
//mod kafka;

use args::ProducerArgs;
use clap::Parser;
use log::info;

fn main() {
    // Initialize logging
    env_logger::init();

    // Parse command-line arguments
    let args = ProducerArgs::parse();

    // Log a few key parameters
    info!("topic: {}", args.topic);
    info!("bootstrap: {}", args.bootstrap);
    info!("messageLocation: {}", args.message_location);
    info!("runOnce: {}", args.run_once);
    info!("maxCycles: {}", args.max_cycles);
    info!("delayInMillis: {}", args.delay_millis);
    info!("noDeleteFiles: {}", args.no_delete_files);

    // Build the directory poller
    let poller = file::DirectoryPoller::builder()
        .keep_running(!args.run_once)
        .delete_files(!args.no_delete_files)
        .poll_interval_millis(args.delay_millis)
        .max_poll_cycles(args.max_cycles)
        .build();

    // poll directory
    match poller.poll_directory(&args.message_location) {
        Ok(_) => info!("Directory polling completed successfully"),
        Err(e) => eprintln!("Error polling directory: {}", e),
    }
}
