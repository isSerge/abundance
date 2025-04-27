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

// fn str_to_fixed_string<const CAP: usize>(
//     s: &str,
// ) -> Result<FixedCapacityStringU16<CAP>, ContractError> {
//     let bytes = s.as_bytes();
//     let mut fixed_string = FixedCapacityStringU16::<CAP>::default();
//     let inner_bytes = fixed_string.deref_mut();

//     if inner_bytes.copy_from(bytes) {
//         Ok(fixed_string)
//     } else {
//         Err(ContractError::BadInput)
//     }
// }

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
        #[input] from: &Address,
        #[input] to: &Address,
        #[input] token_id: &TokenId,
    ) -> Result<(), ContractError> {
        env.example_nft_transfer_from(
            MethodContext::Replace,
            env.own_address(),
            from,
            to,
            token_id,
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
            approved_addr,
            token_id,
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
    fn balance_of(
        #[env] env: &Env<'_>,
        #[input] address: &Address,
        // #[output] token_ids: &mut VariableElements<TokenId>,
    ) -> Result<(), ContractError> {
        env.example_nft_balance_of(env.own_address())
    }

    #[view]
    fn owner_of(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError> {
        env.example_nft_owner_of(env.own_address(), &env.own_address(), token_id)
    }

    #[view]
    fn get_approved(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError> {
        env.example_nft_get_approved(env.own_address(), &env.own_address(), token_id)
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
        #[slot] from: &mut MaybeData<OwnershipSlot>,
        #[slot] to: &mut MaybeData<OwnershipSlot>,
        #[input] token_id: &TokenId, // do we need this?
    ) -> Result<(), ContractError> {
        // Check if the token exists
        if to.get().is_none() {
            return Err(ContractError::NotFound);
        }

        // Check if the caller is the owner of the token or the approved address
        let is_owner = env.caller() == from.get().map_or(Address::default(), |slot| slot.owner);
        let is_approved =
            from.get().map_or(Address::default(), |slot| slot.approved) == env.caller();

        if !(is_owner || is_approved) {
            return Err(ContractError::Forbidden);
        }

        let from_slot = from.get_mut_or_default();
        let to_slot = to.get_mut_or_default();

        // Transfer the token
        from_slot.owner = Address::default();
        to_slot.owner = env.own_address();

        // Reset the approved address
        from_slot.approved = Address::default();
        to_slot.approved = Address::default();

        Ok(())
    }

    #[update]
    pub fn approve(
        #[env] env: &mut Env<'_>,
        #[slot] (from_addr, from): (&Address, &mut MaybeData<OwnershipSlot>),
        #[input] approved_addr: &Address,
        #[input] token_id: &TokenId, // do we need this?
    ) -> Result<(), ContractError> {
        // Check if the token exists
        let exists = env.example_nft_exists(env.context(), from_addr)?;
        // is this the right way?
        if exists != Bool::from(true) {
            return Err(ContractError::NotFound);
        }

        // Check if the caller is the owner of the token
        let is_owner = env.caller() == from.get().map_or(Address::default(), |slot| slot.owner);
        if !is_owner {
            return Err(ContractError::Forbidden);
        }

        let from_slot = from.get_mut_or_default();
        from_slot.approved = *approved_addr;

        Ok(())
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
        Ok(Bool::from(ownership_slot.get().is_none()))
    }

    #[view]
    pub fn owner_of(
        #[env] env: &Env<'_>,
        #[slot] ownership_slot: &MaybeData<OwnershipSlot>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError> {
        ownership_slot
            .get()
            .map(|d| d.owner)
            .ok_or(ContractError::NotFound)
    }

    #[view]
    pub fn get_approved(
        #[env] env: &Env<'_>,
        #[slot] ownership_slot: &MaybeData<OwnershipSlot>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError> {
        ownership_slot
            .get()
            .map(|d| d.approved)
            .ok_or(ContractError::NotFound)
    }
}
