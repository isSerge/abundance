#![no_std]

use ab_contracts_common::env::{Env, MethodContext};
use ab_contracts_common::{Address, ContractError};
use ab_contracts_io_type::bool::Bool;
// use ab_contracts_io_type::fixed_capacity_string::FixedCapacityStringU16;
use ab_contracts_io_type::maybe_data::MaybeData;
use ab_contracts_io_type::trivial_type::TrivialType;
use ab_contracts_io_type::variable_elements::VariableElements;
use ab_contracts_macros::contract;
use ab_contracts_standards::non_fungible::{
    // NFT_NAME_MAX_CAPACITY, NFT_SYMBOL_MAX_CAPACITY, NFT_TOKEN_URI_MAX_CAPACITY,
    NonFungible,
    TokenId,
};

#[derive(Debug, Default, Copy, Clone, TrivialType)]
#[repr(C)]
pub struct ExampleNft {
    // TODO: add name and symbol
}

/// --- Slot data ---
#[derive(Debug, Default, Copy, Clone, TrivialType)]
#[repr(C)]
pub struct OwnershipSlot {
    pub owner: Address,
    pub approved: Address,
}
type OwnerTokensSlot = VariableElements<TokenId>;

#[contract]
impl NonFungible for ExampleNft {
    /// --- Transfer methods ---
    #[update]
    fn transfer_from(
        #[env] env: &mut Env<'_>,
        #[input] token_id: &TokenId,
        #[input] from: &Address,
        #[input] to: &Address,
    ) -> Result<(), ContractError> {
        // TODO: check for ownership and approval

        env.example_nft_transfer_from(
            MethodContext::Replace,
            env.own_address(),
            token_id,
            from,
            to,
        )
    }

    /// --- Approval methods ---
    #[update]
    fn approve(
        #[env] env: &mut Env<'_>,
        #[input] approved_addr: &Address,
        #[input] token_id: &TokenId,
    ) -> Result<(), ContractError> {
        env.example_nft_approve(
            MethodContext::Replace,
            env.own_address(),
            &env.caller(),
            token_id,
            approved_addr,
        )
    }

    #[update]
    fn set_approval_for_all(
        #[env] env: &mut Env<'_>,
        #[input] operator: &Address,
        #[input] approved: &Bool,
    ) -> Result<(), ContractError> {
        env.example_nft_set_approval_for_all(
            MethodContext::Replace,
            env.own_address(),
            operator,
            approved,
        )
    }

    /// --- Query methods ---
    #[view]
    fn balance_of(#[env] env: &Env<'_>, #[input] address: &Address) -> Result<(), ContractError> {
        env.example_nft_balance_of(env.own_address(), *address)
    }

    #[view]
    fn owner_of(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError> {
        env.example_nft_owner_of(env.own_address(), token_id)
    }

    #[view]
    fn get_approved(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError> {
        env.example_nft_get_approved(env.own_address(), token_id)
    }

    #[view]
    fn is_approved_for_all(
        #[env] env: &Env<'_>,
        #[input] owner: &Address,
        #[input] operator: &Address,
    ) -> Result<Bool, ContractError> {
        unimplemented!();
    }

    // #[view]
    // fn token_uri(
    //     #[env] env: &Env<'_>,
    //     #[input] _token_id: &TokenId,
    // ) -> Result<FixedCapacityStringU16<NFT_TOKEN_URI_MAX_CAPACITY>, ContractError> {
    //     unimplemented!();
    // }

    // #[view]
    // fn name(
    //     #[env] env: &Env<'_>,
    // ) -> Result<FixedCapacityStringU16<NFT_NAME_MAX_CAPACITY>, ContractError> {
    //     env.example_nft_get_name(env.own_address())
    // }

    // #[view]
    // fn symbol(
    //     #[env] env: &Env<'_>,
    // ) -> Result<FixedCapacityStringU16<NFT_SYMBOL_MAX_CAPACITY>, ContractError> {
    //     unimplemented!();
    // }
}

#[contract]
impl ExampleNft {
    #[init]
    pub fn new() -> Result<Self, ContractError> {
        Ok(Self::default())
    }

    // #[view]
    // pub fn get_name(&self) -> Result<FixedCapacityStringU16<NFT_NAME_MAX_CAPACITY>, ContractError> {
    //     self.name
    // }

    #[update]
    pub fn transfer_from(
        #[env] env: &mut Env<'_>,
        #[slot] token_slot: &MaybeData<OwnershipSlot>,
        #[input] from: &Address,
        #[input] to: &Address,
    ) -> Result<(), ContractError> {
        unimplemented!();
    }

    #[update]
    pub fn approve(#[env] env: &mut Env<'_>) -> Result<(), ContractError> {
        unimplemented!();
    }

    #[update]
    pub fn set_approval_for_all(
        #[env] env: &mut Env<'_>,
        #[input] operator: &Address,
        #[input] approved: &Bool,
    ) -> Result<(), ContractError> {
        unimplemented!();
    }

    // #[view]
    // pub fn balance_of(
    //     #[env] env: &Env<'_>,
    //     #[slot] owner_slot: &MaybeData<OwnerTokensSlot>,
    //     #[output] result: &mut MaybeData<VariableElements<TokenId>>,
    // ) -> Result<(), ContractError> {
    //     unimplemented!();
    // }
    #[view]
    pub fn balance_of() -> Result<(), ContractError> {
        unimplemented!();
    }

    #[view]
    pub fn exists(
        #[slot] ownership_slot: &MaybeData<OwnershipSlot>,
    ) -> Result<Bool, ContractError> {
        let item = ownership_slot.get();
        Ok(Bool::new(item.is_some()))
    }

    #[view]
    pub fn owner_of(
        #[slot] ownership_slot: &MaybeData<OwnershipSlot>,
    ) -> Result<Address, ContractError> {
        ownership_slot
            .get()
            .map(|d| d.owner)
            .ok_or(ContractError::NotFound)
    }

    #[view]
    pub fn get_approved(
        #[slot] ownership_slot: &MaybeData<OwnershipSlot>,
    ) -> Result<Address, ContractError> {
        ownership_slot
            .get()
            .map(|d| d.approved)
            .ok_or(ContractError::NotFound)
    }
}
