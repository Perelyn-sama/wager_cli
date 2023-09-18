use crate::contracts::bindings::i_wager::IWager;
use ethers::prelude::{builders::ContractCall, *};
use std::sync::Arc;
#[derive(Debug)]
pub struct Wager<M> {
    pub contract: IWager<M>,
}

impl<M> Wager<M> {
    /// Returns a reference to the Airswap Wager contract.
    pub fn contract(&self) -> &IWager<M> {
        &self.contract
    }
}

impl<M: Middleware> Wager<M> {
    /// Creates a new instance from using the provided address.
    pub fn new(client: Arc<M>, address: Address) -> Self {
        let contract = IWager::new(address, client);
        Self { contract }
    }

    pub fn add_players(&self, players: Vec<Address>) -> ContractCall<M, ()> {
        let wager = self.contract();
        wager.add_players(players)
    }

    pub fn remove_players(&self, players: Vec<Address>) -> ContractCall<M, ()> {
        let wager = self.contract();
        wager.remove_players(players)
    }

    pub fn set_bet_amount(&self, amount: U256) -> ContractCall<M, ()> {
        let wager = self.contract();
        wager.set_bet_amount(amount)
    }

    pub fn submit_bet(&self, bet_hash: [u8; 32]) -> ContractCall<M, ()> {
        let wager = self.contract();
        wager.submit_bet(bet_hash)
    }

    pub fn provide_outcome(&self, bet_hash: [u8; 32]) -> ContractCall<M, ()> {
        let wager = self.contract();
        wager.provide_outcome(bet_hash)
    }

    pub fn balance(&self) -> ContractCall<M, U256> {
        let wager = self.contract();
        wager.balance()
    }

    pub fn pay_winner(&self) -> ContractCall<M, ()> {
        let wager = self.contract();
        wager.pay_winner()
    }
}

impl<M> std::ops::Deref for Wager<M> {
    type Target = IWager<M>;
    fn deref(&self) -> &Self::Target {
        self.contract()
    }
}
