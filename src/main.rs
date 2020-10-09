mod logging;
mod test;

use log::{debug, error, info, trace, warn};
use crate::logging::setup_log;

fn main() {
    // Logging
    setup_log();

    info!("Starting HTA!");
}
