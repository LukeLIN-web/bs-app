use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct LoginRequest {
    pub mail: String,
    pub password: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub mail: String,
    pub name: String,
    pub password: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct CreateDeviceRequest {
    pub login_token: String,
    /// mail - user mail address
    pub mail: String,
    /// id - device id
    pub id: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct RemoveDeviceRequest {
    pub login_token: String,
    /// mail - user mail address
    pub mail: String,
    /// id - device id
    pub id: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct FetchDeviceRequest {
    pub login_token: String,
    /// id - device id
    pub id: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct ModifyDeviceRequest {
    pub login_token: String,
    /// id - device id
    pub id: String,
    /// mail - user mail address
    pub name: String,
    pub info: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct FetchDeviceListRequest {
    pub login_token: String,
    /// mail - user mail address
    pub mail: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchMessageListRequest {
    pub login_token: String,
    /// id - device id
    pub id: String,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

impl Default for FetchMessageListRequest {
    fn default() -> Self {
        Self {
            login_token: String::default(),
            id: String::default(),
            start_timestamp: 0,
            end_timestamp: std::u64::MAX,
        }
    }
}
