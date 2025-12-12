use serde::Deserialize;

#[derive(Debug, Deserialize)]

struct RawKiwibankInputAccount {
    #[serde(rename = "Account number")]
    account_number: String,

    #[serde(rename = "Effective Date")]
    effective_date: String,

    #[serde(rename = "Transaction Date")]
    transaction_date: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Transaction Code")]
    transaction_code: String,

    #[serde(rename = "Particulars")]
    particulars: String,

    #[serde(rename = "Code")]
    code: String,

    #[serde(rename = "Reference")]
    reference: String,

    #[serde(rename = "Other Party Name")]
    other_party_name: String,

    #[serde(rename = "Other Party Account Number")]
    other_party_account_number: String,

    #[serde(rename = "Other Party Particulars")]
    other_party_particulars: String,

    #[serde(rename = "Other Party Code")]
    other_party_code: String,

    #[serde(rename = "Other Party Reference")]
    other_party_reference: String,

    #[serde(rename = "Amount")]
    amount: String,

    #[serde(rename = "Balance")]
    balance: String,
}
