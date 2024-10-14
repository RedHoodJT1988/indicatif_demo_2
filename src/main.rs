use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
fn main() {
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{spinner:.green} {msg}");
    let pb = ProgressBar::new_spinner();
    // calculate the duration for 120 ticks per second
    let tick_duration = Duration::from_millis(1000 / 120);
    pb.enable_steady_tick(tick_duration);
    // set the style, providing a default if an error occurs
    pb.set_style(spinner_style.unwrap_or(ProgressStyle::default_spinner()));
    // simulate a long-running task
    for _ in 0..50 {
        pb.set_message("Processing...");
        std::thread::sleep(std::time::Duration::from_millis(90));
    }
    pb.finish_with_message("Done!");
}