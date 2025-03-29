use crate::structs::account::Account;

pub struct Client {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub accounts: Vec<Account>,
}
