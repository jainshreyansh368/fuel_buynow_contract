library external_interface;

abi externalAbi {
    /// Returns the user which owns the specified token.
    ///
    /// # Arguments
    ///
    /// * `token_id` - The unique identifier of the token.
    ///
    /// # Reverts
    ///
    /// * When there is no owner for the `token_id`.
    #[storage(read)]
    fn owner_of(token_id: u64) -> Identity;

    /// Transfers ownership of the specified token from one user to another.
    ///
    /// Transfers can occur under one of three conditions:
    /// 1. The token's owner is transfering the token.
    /// 2. The token's approved user is transfering the token.
    /// 3. The token's owner has a user set as an operator and is transfering the token.
    ///
    /// # Arguments
    ///
    /// * `from` - The user which currently owns the token to be transfered.
    /// * `to` - The user which the ownership of the token should be set to.
    /// * `token_id` - The unique identifier of the token which should be transfered.
    ///
    /// # Reverts
    ///
    /// * When the `token_id` does not map to an existing token.
    /// * When the sender is not the owner of the token.
    /// * When the sender is not approved to transfer the token on the owner's behalf.
    /// * When the sender is not approved to transfer all tokens on the owner's behalf.
    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64);

    /// Gives approval to the `approved` user to transfer a specific token on another user's behalf.
    ///
    /// To revoke approval the approved user should be `None`.
    ///
    /// # Arguments
    ///
    /// * `approved` - The user which will be allowed to transfer the token on the owner's behalf.
    /// * `token_id` - The unique identifier of the token which the owner is giving approval for.
    ///
    /// # Reverts
    ///
    /// * When `token_id` does not map to an existing token.
    /// * When the sender is not the token's owner.
    #[storage(read, write)]
    fn approve(approved: Identity, token_id: u64);
}
