/// Credentials to use for KMS requests.

#[derive(Debug, Clone)]
pub struct Credentials {
	pub access_key_id: String,
	pub secret_access_key: String,
	pub session_token: Option<String>,
}

impl Credentials {
	/// Creates a new set of KMS credentials.
	pub fn new(
		access_key_id: impl Into<String>,
		secret_access_key: impl Into<String>,
		session_token: Option<String>,
	) -> Self {
		Self {
			session_token,
			access_key_id: access_key_id.into(),
			secret_access_key: secret_access_key.into(),
		}
	}
}