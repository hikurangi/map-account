use serde::Serialize;

#[derive(Debug, Serialize)]
struct RawKiwibankOutputAccount {
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
