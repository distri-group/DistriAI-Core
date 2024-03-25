use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Order {
    pub order_id: [u8; 16],
    /// The buyer of this order.
    pub buyer: Pubkey,
    /// The seller of this order.
    pub seller: Pubkey,
    /// The machine id of this order.
    pub machine_id: [u8; 16],
    /// The price of this order.
    pub price: u64,
    /// The duration hours of this order.
    pub duration: u32,
    /// The total amount of this order.
    pub total: u64,
    /// The metadata by json format of this order.
    #[max_len(2048)]
    pub metadata: String,
    /// The status of this order.
    pub status: OrderStatus,
    /// The order time of this order.
    pub order_time: i64,
    /// The start time of this order.
    pub start_time: i64,
    /// The refund time of this order.
    pub refund_time: i64,
}

impl Order {
    pub const METADATA_MAX_LENGTH: usize = 2048;
}

#[account]
#[derive(InitSpace)]
pub struct OrderNew {
    pub order_id: [u8; 16], // 16
    /// The buyer of this order.
    pub buyer: Pubkey, // 32
    /// The seller of this order.
    pub seller: Pubkey, // 32
    /// The machine id of this order.
    pub machine_id: [u8; 16], // 16
    /// The price of this order.
    pub price: u64, //8
    /// The duration hours of this order.
    pub duration: u32, // 4
    /// The total amount of this order.
    pub total: u64, //8
    /// The metadata by json format of this order.
    #[max_len(2048)]
    pub metadata: String, // 4 + 2048
    /// The status of this order.
    pub status: OrderStatus, // 1 + 1
    /// The order time of this order.
    pub order_time: i64, // 8
    /// The start time of this order.
    pub start_time: i64, // 8
    /// The refund time of this order.
    pub refund_time: i64, // 8
}

/// OrderStatus holds the current state of the order.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum OrderStatus {
    /// This order is preparing. The state of the machine is `Renting`.
    Preparing,
    /// This order is in training. The state of the machine is `Renting`.
    Training,
    /// This order was completed successfully.
    Completed,
    /// This order was failed.
    Failed,
    /// This order was refunded.
    Refunded,
}
