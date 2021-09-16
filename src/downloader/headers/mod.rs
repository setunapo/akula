pub mod header_slices;

pub mod fetch_receive_stage;
pub mod fetch_request_stage;
pub mod preverified_hashes_config;
pub mod refill_stage;
pub mod retry_stage;
pub mod save_stage;
pub mod verify_stage;

#[cfg(feature = "crossterm")]
pub mod ui_crossterm;
#[cfg(feature = "crossterm")]
pub use ui_crossterm::HeaderSlicesView;

#[cfg(not(feature = "crossterm"))]
pub mod ui_tracing;

#[cfg(not(feature = "crossterm"))]
pub use ui_tracing::HeaderSlicesView;