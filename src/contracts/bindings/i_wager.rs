pub use i_wager::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_wager {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IWager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_players\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPlayers\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"payWinner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_betHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"provideOutcome\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_players\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removePlayers\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBetAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_betHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"submitBet\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IWAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IWager<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IWager<M> {
        fn clone(&self) -> Self {
            IWager(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IWager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IWager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IWager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IWager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IWAGER_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `addPlayers` (0x493ff1d6) function"]
        pub fn add_players(
            &self,
            players: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 63, 241, 214], players)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balance` (0xb69ef8a8) function"]
        pub fn balance(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 158, 248, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `payWinner` (0xbe71248a) function"]
        pub fn pay_winner(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 113, 36, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `provideOutcome` (0xa62d1a1b) function"]
        pub fn provide_outcome(
            &self,
            bet_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 45, 26, 27], bet_hash)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removePlayers` (0xdc398933) function"]
        pub fn remove_players(
            &self,
            players: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 57, 137, 51], players)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBetAmount` (0x53a79d74) function"]
        pub fn set_bet_amount(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 167, 157, 116], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitBet` (0xd99ccc2d) function"]
        pub fn submit_bet(
            &self,
            bet_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 156, 204, 45], bet_hash)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IWager<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `addPlayers` function with signature `addPlayers(address[])` and selector `[73, 63, 241, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addPlayers", abi = "addPlayers(address[])")]
    pub struct AddPlayersCall {
        pub players: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `balance` function with signature `balance()` and selector `[182, 158, 248, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balance", abi = "balance()")]
    pub struct BalanceCall;
    #[doc = "Container type for all input parameters for the `payWinner` function with signature `payWinner()` and selector `[190, 113, 36, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "payWinner", abi = "payWinner()")]
    pub struct PayWinnerCall;
    #[doc = "Container type for all input parameters for the `provideOutcome` function with signature `provideOutcome(bytes32)` and selector `[166, 45, 26, 27]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "provideOutcome", abi = "provideOutcome(bytes32)")]
    pub struct ProvideOutcomeCall {
        pub bet_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `removePlayers` function with signature `removePlayers(address[])` and selector `[220, 57, 137, 51]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "removePlayers", abi = "removePlayers(address[])")]
    pub struct RemovePlayersCall {
        pub players: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `setBetAmount` function with signature `setBetAmount(uint256)` and selector `[83, 167, 157, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setBetAmount", abi = "setBetAmount(uint256)")]
    pub struct SetBetAmountCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `submitBet` function with signature `submitBet(bytes32)` and selector `[217, 156, 204, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "submitBet", abi = "submitBet(bytes32)")]
    pub struct SubmitBetCall {
        pub bet_hash: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IWagerCalls {
        AddPlayers(AddPlayersCall),
        Balance(BalanceCall),
        PayWinner(PayWinnerCall),
        ProvideOutcome(ProvideOutcomeCall),
        RemovePlayers(RemovePlayersCall),
        SetBetAmount(SetBetAmountCall),
        SubmitBet(SubmitBetCall),
    }
    impl ethers::core::abi::AbiDecode for IWagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddPlayersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWagerCalls::AddPlayers(decoded));
            }
            if let Ok(decoded) =
                <BalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWagerCalls::Balance(decoded));
            }
            if let Ok(decoded) =
                <PayWinnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWagerCalls::PayWinner(decoded));
            }
            if let Ok(decoded) =
                <ProvideOutcomeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWagerCalls::ProvideOutcome(decoded));
            }
            if let Ok(decoded) =
                <RemovePlayersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWagerCalls::RemovePlayers(decoded));
            }
            if let Ok(decoded) =
                <SetBetAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWagerCalls::SetBetAmount(decoded));
            }
            if let Ok(decoded) =
                <SubmitBetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWagerCalls::SubmitBet(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IWagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IWagerCalls::AddPlayers(element) => element.encode(),
                IWagerCalls::Balance(element) => element.encode(),
                IWagerCalls::PayWinner(element) => element.encode(),
                IWagerCalls::ProvideOutcome(element) => element.encode(),
                IWagerCalls::RemovePlayers(element) => element.encode(),
                IWagerCalls::SetBetAmount(element) => element.encode(),
                IWagerCalls::SubmitBet(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IWagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IWagerCalls::AddPlayers(element) => element.fmt(f),
                IWagerCalls::Balance(element) => element.fmt(f),
                IWagerCalls::PayWinner(element) => element.fmt(f),
                IWagerCalls::ProvideOutcome(element) => element.fmt(f),
                IWagerCalls::RemovePlayers(element) => element.fmt(f),
                IWagerCalls::SetBetAmount(element) => element.fmt(f),
                IWagerCalls::SubmitBet(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddPlayersCall> for IWagerCalls {
        fn from(var: AddPlayersCall) -> Self {
            IWagerCalls::AddPlayers(var)
        }
    }
    impl ::std::convert::From<BalanceCall> for IWagerCalls {
        fn from(var: BalanceCall) -> Self {
            IWagerCalls::Balance(var)
        }
    }
    impl ::std::convert::From<PayWinnerCall> for IWagerCalls {
        fn from(var: PayWinnerCall) -> Self {
            IWagerCalls::PayWinner(var)
        }
    }
    impl ::std::convert::From<ProvideOutcomeCall> for IWagerCalls {
        fn from(var: ProvideOutcomeCall) -> Self {
            IWagerCalls::ProvideOutcome(var)
        }
    }
    impl ::std::convert::From<RemovePlayersCall> for IWagerCalls {
        fn from(var: RemovePlayersCall) -> Self {
            IWagerCalls::RemovePlayers(var)
        }
    }
    impl ::std::convert::From<SetBetAmountCall> for IWagerCalls {
        fn from(var: SetBetAmountCall) -> Self {
            IWagerCalls::SetBetAmount(var)
        }
    }
    impl ::std::convert::From<SubmitBetCall> for IWagerCalls {
        fn from(var: SubmitBetCall) -> Self {
            IWagerCalls::SubmitBet(var)
        }
    }
    #[doc = "Container type for all return fields from the `balance` function with signature `balance()` and selector `[182, 158, 248, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceReturn(pub ethers::core::types::U256);
}
