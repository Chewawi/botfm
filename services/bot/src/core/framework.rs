use std::sync::Arc;
use std::time::Duration;

use common::config::CONFIG;
use lumi::{Framework, FrameworkOptions, PrefixFrameworkOptions, EditTracker};

use crate::commands;
use crate::core::hooks::{pre_command, post_command};
use crate::core::structs::{Data, Error};

/// Initialize the lumi framework with pre_command and post_command hooks
/// for command timing and metrics.
pub fn init_framework() -> Framework<Data, Error> {
    Framework::new(FrameworkOptions {
        prefix_options: PrefixFrameworkOptions {
            prefix: Some(CONFIG.prefix.get().into()),
            mention_as_prefix: true,
            case_insensitive_commands: true,
            edit_tracker: Some(Arc::new(EditTracker::for_timespan(
                Duration::from_secs(600),
            ))),
            ..Default::default()
        },
        commands: commands::register_all_commands(),
        pre_command,
        post_command,
        ..Default::default()
    })
}