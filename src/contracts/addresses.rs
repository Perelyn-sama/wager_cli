#![allow(dead_code)]
use ethers_contract::Lazy;
use ethers_core::types::{Address, Chain};
use serde::Deserialize;
use std::{borrow::Borrow, collections::HashMap};

const ADDRESSES_JSON: &str = include_str!("./addresses.json");

static ADDRESS_BOOK: Lazy<HashMap<String, Contract>> =
    Lazy::new(|| serde_json::from_str(ADDRESSES_JSON).unwrap());

/// Wrapper around a hash map that maps a [Chain] to the contract's deployed address on that chain.
#[derive(Clone, Debug, Deserialize)]
pub struct Contract {
    addresses: HashMap<Chain, Address>,
}

impl Contract {
    /// Returns the address of the contract on the specified chain. Returns None if the contract's
    /// address is not found in the addressbook.
    pub fn address<C: Borrow<Chain>>(&self, chain: C) -> Option<Address> {
        self.addresses.get(chain.borrow()).copied()
    }
}

/// Fetch the addressbook for a contract by its name. If the contract name is not a part of the
/// address book we return None.
pub fn try_contract<S: Borrow<str>>(name: S) -> Option<&'static Contract> {
    ADDRESS_BOOK.get(name.borrow())
}

/// Fetch the address for a contract by its name and chain. If the contract name is not a part of
/// the address book we return None.
pub fn try_address<S: Borrow<str>, C: Borrow<Chain>>(name: S, chain: C) -> Option<Address> {
    let contract = try_contract(name);
    contract.and_then(|contract| contract.address(chain))
}

/// Fetch the addressbook for a contract by its name. If the contract name is not a part of the
/// address book we panic.
pub fn contract<S: Borrow<str>>(name: S) -> &'static Contract {
    let name = name.borrow();
    try_contract(name).unwrap_or_else(|| panic!("\"{name}\" is not present in addressbook"))
}

/// Fetch the address for a contract by its name and chain. If the contract name is not a part of
/// the address book we panic.
pub fn address<S: Borrow<str>, C: Borrow<Chain>>(name: S, chain: C) -> Address {
    let name = name.borrow();
    let chain = chain.borrow();
    let contract = contract(name);
    contract.address(chain).unwrap_or_else(|| {
        panic!("\"{chain:?}\" for contract \"{name}\" is not present in addressbook")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contracts() {
        assert!(try_contract("DAI").is_some());
        assert!(try_contract("USDC").is_some());
        assert!(try_contract("rand").is_none());
    }

    #[test]
    fn test_addresses() {
        assert!(try_contract("DAI")
            .unwrap()
            .address(Chain::Mainnet)
            .is_some());
        assert!(try_contract("DAI")
            .unwrap()
            .address(Chain::MoonbeamDev)
            .is_none());

        assert!(try_address("DAI", Chain::Mainnet).is_some());
        assert!(try_address("DAI", Chain::MoonbeamDev).is_none());
    }
}
