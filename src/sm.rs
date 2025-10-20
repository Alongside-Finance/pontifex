use aws_sdk_secretsmanager::config::SharedCredentialsProvider;
use aws_smithy_http_client::hyper_014::HyperClientBuilder;
use aws_types::SdkConfig;
use tokio_vsock::VsockAddr;

use crate::utils::http::vsock_proxy;
use crate::utils::credentials::Credentials;
use crate::kms::VSOCK_PROXY_CID;

/// Creates a new KMS client.
#[must_use]
pub fn client(
	config: &SdkConfig,
	credentials: Credentials,
	vsock_proxy_port: u32,
) -> aws_sdk_secretsmanager::Client {
	let builder = config
		.to_builder()
		.credentials_provider(SharedCredentialsProvider::new(
			aws_sdk_secretsmanager::config::Credentials::new(
				credentials.access_key_id,
				credentials.secret_access_key,
				credentials.session_token,
				None,
				"SDK",
			),
		))
		.http_client(HyperClientBuilder::new().build(vsock_proxy(VsockAddr::new(
			VSOCK_PROXY_CID,
			vsock_proxy_port,
		))))
		.build();

		aws_sdk_secretsmanager::Client::new(&builder)
}
