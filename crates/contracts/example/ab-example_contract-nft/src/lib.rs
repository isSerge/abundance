#![no_std]

use ab_contracts_common::env::{Env, MethodContext};
use ab_contracts_common::{Address, ContractError};
use ab_contracts_io_type::bool::Bool;
// use ab_contracts_io_type::fixed_capacity_string::FixedCapacityStringU16;
// use ab_contracts_io_type::maybe_data::MaybeData;
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
    // name: FixedCapacityStringU16<NFT_NAME_MAX_CAPACITY>,
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
pub struct ExampleNftSlotData {
    pub owner: Address,
    pub approved: Address,
}

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
        if !(env.context() == from || env.caller() == from || env.caller() == env.own_address()) {
            return Err(ContractError::Forbidden);
        }

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
        // Check if the caller is the owner of the token
        if env.caller() != env.own_address() {
            return Err(ContractError::Forbidden);
        }

        // Check if the token exists
        if env.example_nft_exists(env.own_address(), token_id)?.get() == false {
            return Err(ContractError::NotFound);
        }

        // Check if the approved address is not the owner of the token
        if env.example_nft_owner_of(env.own_address(), token_id)? == *approved_addr {
            return Err(ContractError::BadInput);
        }

        env.example_nft_approve(
            MethodContext::Replace,
            env.own_address(),
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
        // Check if the caller is the owner of the token
        if env.caller() != env.own_address() {
            return Err(ContractError::Forbidden);
        }

        // Check if operator is not the owner of the token
        if env.caller() == operator {
            return Err(ContractError::BadInput);
        }

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
        #[output] token_ids: &mut VariableElements<TokenId>,
    ) -> Result<(), ContractError> {
        unimplemented!();
    }

    #[view]
    fn owner_of(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError> {
        // Check if the token exists
        if env.example_nft_exists(env.own_address(), token_id)?.get() == false {
            return Err(ContractError::NotFound);
        }

        env.example_nft_owner_of(env.own_address(), token_id)
    }

    #[view]
    fn get_approved(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError> {
        // Check if the token exists
        if env.example_nft_exists(env.own_address(), token_id)?.get() == false {
            return Err(ContractError::NotFound);
        }

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
        #[input] from: &Address,
        #[input] to: &Address,
        #[input] token_id: &TokenId,
    ) -> Result<(), ContractError> {
        unimplemented!();
    }

    #[update]
    pub fn approve(
        #[env] env: &mut Env<'_>,
        #[input] approved_addr: &Address,
        #[input] token_id: &TokenId,
    ) -> Result<(), ContractError> {
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
    //     #[slot] owner_tokens_slot: &MaybeData<OwnerTokensSlot>,
    //     #[output] result: &mut VariableElements<TokenId>,
    // ) -> Result<(), ContractError> {
    //     // Get the list from the slot

    //     Ok(())
    // }

    #[view]
    pub fn exists(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) -> Result<Bool, ContractError> {
        unimplemented!();
    }

    #[view]
    pub fn owner_of(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError> {
        unimplemented!();
    }

    #[view]
    pub fn get_approved(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError> {
        unimplemented!();
    }
}
