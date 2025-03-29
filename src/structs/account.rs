use crate::structs::client::Client;
pub enum AccountType {
    Chequing,
    Savings,
    TFSA,
    RRSP
}

pub struct Account {
    pub active: bool,
    pub owner: Client,
    pub account_type: AccountType,
}

