mod args;
//mod file;
//mod kafka;
//mod content;

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
    info!("delayInMillis: {}", args.delay_millis);
    info!("deleteFiles: {}", !(args.no_delete_files));
}
