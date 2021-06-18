//! [POST /_matrix/identity/v2/validate/email/submitToken](https://matrix.org/docs/spec/identity_service/r0.3.0#post-matrix-identity-v2-validate-email-submittoken)

use ruma_api::ruma_api;
use ruma_identifiers::{ClientSecretBox, SessionIdBox};

ruma_api! {
    metadata: {
        description: "Validate ownership of an email address.",
        method: POST,
        name: "validate_email",
        path: "/_matrix/identity/v2/validate/email/submitToken",
        authentication: AccessToken,
        rate_limited: false,
    }

    request: {
        /// The session ID, generated by the `requestToken` call.
        pub sid: &'a SessionIdBox,

        /// The client secret that was supplied to the `requestToken` call.
        pub client_secret: &'a ClientSecretBox,

        /// The token generated by the `requestToken` call and emailed to the user.
        pub token: &'a str,
    }

    response: {
        /// Whether the validation was successful or not.
        pub success: bool,
    }
}

impl<'a> Request<'a> {
    /// Create a new `Request` with the given session ID, client secret and token.
    pub fn new(sid: &'a SessionIdBox, client_secret: &'a ClientSecretBox, token: &'a str) -> Self {
        Self { sid, client_secret, token }
    }
}

impl Response {
    /// Create a new `Response` with the success status.
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
