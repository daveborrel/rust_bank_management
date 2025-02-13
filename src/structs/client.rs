use crate::structs::account::Account;

pub struct Client {
    pub username: String,
    pub password: String,
    pub firstName: String,
    pub lastName: String,
    pub email: String,
    pub accounts: Vec<Account>,
}
