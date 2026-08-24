//! The server host's integration tests, one binary. Each module was a separate test binary
//! that linked the whole host; one binary links it once.

#[path = "../common/mod.rs"]
mod common;
#[path = "../support/robot_vacuum_fixture.rs"]
mod robot_vacuum_fixture;

mod cli_ai_tools;
mod cli_bundle;
mod examples_are_clean;
mod generator_cli;
mod init_scaffold;
mod kitchen_timer_check;
mod kpar_domain_libraries_embed_smoke;
mod kpar_stdlib_embed_smoke;
mod multi_file_check;
mod robot_vacuum_check;
