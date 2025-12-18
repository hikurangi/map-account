use serde::Serialize;

use crate::types::input::RawKiwibankInputAccount;

#[derive(Debug, Serialize)]
pub struct RawKiwibankOutputAccount {
    #[serde(rename = "Account number")]
    account_number: String,

    #[serde(rename = "Date")]
    date: String,

    #[serde(rename = "Memo/Description")]
    memo_description: String,

    #[serde(rename = "Source Code (payment type)")]
    source_code_payment_type: String,

    #[serde(rename = "TP ref")]
    tp_ref: String,

    #[serde(rename = "TP part")]
    tp_part: String,

    #[serde(rename = "TP code")]
    tp_code: String,

    #[serde(rename = "OP ref")]
    op_ref: String,

    #[serde(rename = "OP part")]
    op_part: String,

    #[serde(rename = "OP code")]
    op_code: String,

    #[serde(rename = "OP name")]
    op_name: String,

    #[serde(rename = "OP Bank Account Number")]
    op_bank_account_number: String,

    #[serde(rename = "Amount (credit)")]
    amount_credit: String,

    #[serde(rename = "Amount (debit)")]
    amount_debit: String,

    #[serde(rename = "Amount")]
    amount: String,

    #[serde(rename = "Balance")]
    balance: String,
}

impl From<RawKiwibankInputAccount> for RawKiwibankOutputAccount {
    fn from(
        RawKiwibankInputAccount {
            account_number,
            effective_date,
            transaction_date: _,
            description,
            transaction_code,
            particulars,
            code,
            reference,
            other_party_name,
            other_party_account_number,
            other_party_particulars,
            other_party_code,
            other_party_reference,
            amount,
            balance,
        }: RawKiwibankInputAccount,
    ) -> Self {
        Self {
            account_number,
            date: effective_date,
            memo_description: description,
            source_code_payment_type: transaction_code,
            tp_ref: reference,
            tp_part: particulars,
            tp_code: code,
            op_ref: other_party_reference,
            op_name: other_party_name,
            op_part: other_party_particulars,
            op_code: other_party_code,
            op_bank_account_number: other_party_account_number,
            amount_credit: String::new(),
            amount_debit: String::new(),
            amount,
            balance,
        }
    }
}
