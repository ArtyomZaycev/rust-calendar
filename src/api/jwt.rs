use jwt_simple::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomClaims {
    #[serde(rename = "uid")]
    pub user_id: i32,
    #[serde(rename = "acc")]
    pub access_level: i32,
    #[serde(rename = "edit")]
    pub edit_rights: bool,
}

fn get_key() -> HS256Key {
    HS256Key::from_bytes(&[1, 2, 3])
}

pub fn jwt_to_string(claims: JWTClaims<CustomClaims>) -> Option<String> {
    get_key().authenticate(claims).ok()
}

pub fn create_jwt(claims: CustomClaims) -> Option<String> {
    jwt_to_string(Claims::with_custom_claims(claims, Duration::from_hours(2)))
}

pub fn verify_jwt(jwt: &str) -> Option<JWTClaims<CustomClaims>> {
    get_key().verify_token::<CustomClaims>(&jwt, None).ok()
}
