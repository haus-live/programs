use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Signer has no authority over the event")]
    InvalidOwner,
    #[msg("Invalid event category.")]
    InvalidEventCategory,
    #[msg("The event has not started yet.")]
    EventNotStarted,
    #[msg("The event has already ended.")]
    EventEnded,
    #[msg("The event has not ended yet.")]
    EventNotEnded,
    #[msg("Invalid event duration. Should be either 15m, 30m or 45m")]  /// CHECK
    EventDurationInvalid,
    #[msg("TGA (Ticket Gated Access)")]
    NoTicket,
}

#[error_code]
pub enum NftVerifierError {
    #[msg("Signer does not own the token account")]
    InvalidOwner,
    #[msg("Token account does not match mint")]
    InvalidMint,
    #[msg("Token account must hold exactly 1 token")]
    InvalidAmount,
    #[msg("Invalid metadata account")]
    InvalidMetadataAccount,
    #[msg("NFT collection is not verified")]
    UnverifiedCollection,
    #[msg("NFT does not belong to the expected collection")]
    InvalidCollection,
    #[msg("NFT has no collection data")]
    NoCollectionData,
}
