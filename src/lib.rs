pub mod contracts;
pub mod wager;

pub mod prelude {
    pub use super::wager::Wager;

    #[cfg(feature = "addresses")]
    pub use super::contracts::addresses::{address, contract, try_address, try_contract};
}
