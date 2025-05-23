//! Utility functions for testing a Supervisor
pub mod fixture_root;
pub mod fs;
pub mod bio_root;
pub mod test_butterfly;
pub mod test_helpers;
pub mod test_sup;

// Re-export the key structs of this package for ergonomics.
pub use self::{fixture_root::FixtureRoot,
               fs::{setup_package_files,
                    FileSystemSnapshot},
               bio_root::BioRoot,
               test_sup::TestSup};
