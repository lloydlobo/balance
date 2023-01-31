use serde::{Deserialize, Serialize};

// - account: expense:grocery amount: 0 budget_month: 500 budget_year: 6000
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Account {
    pub account: String,
    pub amount: i64,
    pub budget_month: i64,
    pub budget_year: i64,
}

// The sign (debit / credit) associated with the offset_account value is the opposite of the sign of
// the value contained in amount field. In the above example transaction, since expense_general was
// debited by 200, the cc_amex account will be credited by the same amount. Transactions that
// involve more than two accounts are expressed in the following manner:
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Transaction {
    pub date: String,
    pub amount: i64,
    pub offset_account: String,
    pub description: String,
    pub account: Account,
}

// https://github.com/ebcrowder/rust_ledger/blob/main/src/ledger.rs
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct BalanceFile {
    pub currency: String,
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}
