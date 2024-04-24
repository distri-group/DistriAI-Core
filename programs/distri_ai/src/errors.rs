use anchor_lang::error_code;

#[error_code]
pub enum DistriAIError {
    /// A string is too long.
    StringTooLong,
    /// The machine/order status is not the expected status.
    IncorrectStatus,
    /// The duration of the order exceeds the maximum available duration of the machine.
    DurationTooMuch,
    /// Period is invalid.
    InvalidPeriod,
    /// Reward has been claimed.
    RepeatClaim,
}
