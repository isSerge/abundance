use ab_contracts_common::env::Env;
use ab_contracts_common::{Address, ContractError};
use ab_contracts_io_type::bool::Bool;
use ab_contracts_macros::contract;
use ab_contracts_io_type::variable_elements::VariableElements;
// use ab_contracts_io_type::fixed_capacity_string::{FixedCapacityStringU8, FixedCapacityStringU16};

// TODO: consider making these constants configurable
/// --- Constants ---
/// The maximum length of the token name, in bytes.
pub const NFT_NAME_MAX_CAPACITY: usize = 64;
/// The maximum length of the token symbol, in bytes.
pub const NFT_SYMBOL_MAX_CAPACITY: usize = 16;
/// The maximum length of the token URI, in bytes.
pub const NFT_TOKEN_URI_MAX_CAPACITY: usize = 256;

// TODO: consider making configurable
/// --- TokenId type ---
pub type TokenId = u64;

/// Non-fungible token trait prototype
#[contract]
pub trait NonFungible {
    // --- Transfer methods ---
    /// Transfer ownership from owner to another address. Handles both owner-initiated and delegated transfers.
    /// MUST be called by the owner of the token or an approved address.
    /// MUST verify that from is the owner of the token.
    #[update]
    fn transfer_from(
        #[env] env: &mut Env<'_>,
        #[input] from: &Address,
        #[input] to: &Address,
        #[input] token_id: &u64,
    ) -> Result<(), ContractError>;

    // --- Approval methods ---
    /// Approve another address to transfer a single token on behalf of the owner.
    /// MUST be called by the owner of the token.
    /// MUST verify that the token exists.
    /// MUST verify that the approved address is not the owner of the token.
    #[update]
    fn approve(
        #[env] env: &mut Env<'_>,
        #[input] approved_addr: &Address,
        #[input] token_id: &TokenId,
    ) -> Result<(), ContractError>;

    /// Grant or revoke approval for an operator to transfer all tokens on behalf of the owner.
    /// MUST be called by the owner of the token.
    /// MUST verify that the operator is not the owner
    #[update]
    fn set_approval_for_all(
        #[env] env: &mut Env<'_>,
        #[input] operator: &Address,
        #[input] approved: &Bool,
    ) -> Result<(), ContractError>;

    // --- Query methods ---
    /// Get the list of tokens owned by an address.
    #[view]
    fn balance_of(
        #[env] env: &Env<'_>,
        #[input] address: &Address,
        #[output] token_ids: &mut VariableElements<TokenId>,
    ) -> Result<(), ContractError>;

    /// Get the owner of a token.
    /// Returns error if token does not exist.
    #[view]
    fn owner_of(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) -> Result<Address, ContractError>;

    /// Get the approved address for a token.
    /// Returns `Address::NULL` if no address is approved.
    /// Returns error if token does not exist.
    #[view]
    fn get_approved(
        #[env] env: &Env<'_>,
        #[input] token_id: &TokenId,
    ) ->  Result<Address, ContractError>;

    /// Check if an address is approved to transfer all tokens on behalf of the owner.
    #[view]
    fn is_approved_for_all(
        #[env] env: &Env<'_>,
        #[input] owner: &Address,
        #[input] operator: &Address,
    ) -> Result<Bool, ContractError>;

    // /// Get the Uniform Resource Identifier (URI) for a token ID.
    // #[view]
    // fn token_uri(
    //     #[env] env: &Env<'_>,
    //     #[input] token_id: &TokenId,
    // ) -> Result<FixedCapacityStringU16<NFT_TOKEN_URI_MAX_CAPACITY>, ContractError>;

    // /// Get the name of the token contract.
    // #[view]
    // fn name(#[env] env: &Env<'_>) -> Result<FixedCapacityStringU16<NFT_NAME_MAX_CAPACITY>, ContractError>;

    // /// Get the symbol of the token contract.
    // #[view]
    // fn symbol(#[env] env: &Env<'_>) -> Result<FixedCapacityStringU16<NFT_SYMBOL_MAX_CAPACITY>, ContractError>;
}
