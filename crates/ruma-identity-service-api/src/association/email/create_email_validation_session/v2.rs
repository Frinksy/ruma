//! [POST /_matrix/identity/v2/validate/email/requestToken](https://matrix.org/docs/spec/identity_service/r0.3.0#post-matrix-identity-v2-validate-email-requesttoken)

use js_int::UInt;
use ruma_api::ruma_api;
use ruma_identifiers::{ClientSecretBox, SessionIdBox};

ruma_api! {
    metadata: {
        description: "Creates a session for validating an email address.",
        method: POST,
        name: "create_email_validation_session",
        path: "/_matrix/identity/v2/validate/email/requestToken",
        authentication: AccessToken,
        rate_limited: false,
    }

    request: {
        /// A unique string generated by the client, and used to identify the validation attempt.
        pub client_secret: &'a ClientSecretBox,

        /// The email address to validate.
        pub email: &'a str,

        /// The server will only send an email if the send_attempt is a number greater than the
        /// most recent one which it has seen, scoped to that email + client_secret pair.
        pub send_attempt: UInt,

        /// When the validation is completed, the identity server will redirect the user to this
        /// URL.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_link: Option<&'a str>,
    }

    response: {
        /// The session ID. Session IDs are opaque strings generated by the identity server.
        pub sid: SessionIdBox,
    }
}

impl<'a> Request<'a> {
    /// Create a new `Request` with the given client secret, email ID, `send_attempt` number, and
    /// the link to redirect to after validation.
    pub fn new(
        client_secret: &'a ClientSecretBox,
        email: &'a str,
        send_attempt: UInt,
        next_link: Option<&'a str>,
    ) -> Self {
        Self { client_secret, email, send_attempt, next_link }
    }
}

impl Response {
    /// Create a new `Response` with the given session ID.
    pub fn new(sid: SessionIdBox) -> Self {
        Self { sid }
    }
}
