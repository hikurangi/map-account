use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawKiwibankInputAccount {
    #[serde(rename = "Account number")]
    pub account_number: String,

    #[serde(rename = "Effective Date")]
    pub effective_date: String,

    #[serde(rename = "Transaction Date")]
    pub transaction_date: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "Transaction Code")]
    pub transaction_code: String,

    #[serde(rename = "Particulars")]
    pub particulars: String,

    #[serde(rename = "Code")]
    pub code: String,

    #[serde(rename = "Reference")]
    pub reference: String,

    #[serde(rename = "Other Party Name")]
    pub other_party_name: String,

    #[serde(rename = "Other Party Account Number")]
    pub other_party_account_number: String,

    #[serde(rename = "Other Party Particulars")]
    pub other_party_particulars: String,

    #[serde(rename = "Other Party Code")]
    pub other_party_code: String,

    #[serde(rename = "Other Party Reference")]
    pub other_party_reference: String,

    #[serde(rename = "Amount")]
    pub amount: String,

    #[serde(rename = "Balance")]
    pub balance: String,
}
