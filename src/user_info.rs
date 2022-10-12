#[derive(Debug)]
pub struct User {
    pub client_id: String,
    pub balance: i32,
    pub access: u8,
    pub live: bool,
}
