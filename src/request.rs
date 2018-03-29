#[derive(Debug, Deserialize)]
pub struct UserCreationRequest {
    pub email:      String,
    pub password:   String,
    pub name:       String,
}

#[derive(Debug, Deserialize)]
pub struct UserUpdateRequest {
    pub email:      Option<String>,
    pub password:   Option<String>,
    pub name:       Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    pub email:      String,
    pub password:   String,
}

#[derive(Debug, Deserialize)]
pub struct WalletCreationRequest {
    pub user_id:    i32,
    pub name:       String,
    pub balance:    i32,
    pub color:      String,
}