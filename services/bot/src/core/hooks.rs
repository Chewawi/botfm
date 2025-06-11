use std::sync::atomic::Ordering::SeqCst;
use std::time::Instant;
use lumi::BoxFuture;
use tracing::{info, instrument};
use crate::core::structs::{Data, Error};

/// Pre-command hook that records the start time of a command execution
/// and stores it in the context for later use.
#[instrument(skip_all)]
pub fn pre_command(ctx: lumi::Context<'_, Data, Error>) -> BoxFuture<'_, ()> {
    Box::pin(async move {
        // Store the start time in the context
        ctx.data().command_started.swap(Instant::now(), SeqCst);

        info!("Command '{}' started execution", ctx.command().name);
    })
}

/// Post-command hook that calculates and logs the execution time of a command.
#[instrument(skip_all)]
pub fn post_command(ctx: lumi::Context<'_, Data, Error>) -> BoxFuture<'_, ()> {
    Box::pin(async move {
        // Get the start time from the context
        let start_time = ctx.data().command_started.load(SeqCst);
        // Calculate the execution time
        let execution_time = start_time.elapsed();
        
        // Log the execution time and other metrics
        info!(
            "Command '{}' completed in {:.2?}",
            ctx.command().name,
            execution_time,
        );
    })
}