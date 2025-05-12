use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid event category.")]
    InvalidEventCategory,
    #[msg("The event has not started yet.")]
    EventNotStarted,
    #[msg("The event has already ended.")]
    EventEnded,
    #[msg("The event has not ended yet.")]
    EventNotEnded,
    #[msg("No tickets left to sell.")]
    TicketLimitExceeded,
    #[msg("Invalid event duration. Should be either 15m, 30m or 45m")]  /// CHECK
    EventDurationInvalid,
    #[msg("TGA (Ticket Gated Access)")]
    NoTicket,
}
