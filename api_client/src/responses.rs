use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AuthResponse {
    token: String,
}

#[derive(Deserialize, Debug)]
pub struct CompanyResponse {
    pub correlationId: String,
    pub totalSize: u32,
    pub companies: Vec<Company>,
}

#[derive(Deserialize, Debug)]
pub struct Company {
    pub id: String,
    pub country: String,
    pub regNo: String,
    pub name: String,
    pub status: String,
    pub statusDescription: String,

    pub vatNo: Option<Vec<String>>,
    pub tradingNames: Option<Vec<String>>,
    pub phoneNumbers: Option<Vec<String>>,

    pub address: Address,
}

#[derive(Deserialize, Debug)]
pub struct Address {
    pub simpleValue: String,
    pub street: String,
    pub city: String,
    pub postCode: String,
    pub houseNo: String,
}
