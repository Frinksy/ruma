//! [GET /_matrix/identity/v2/validate/msisdn/submitToken](https://matrix.org/docs/spec/identity_service/r0.3.0#get-matrix-identity-v2-validate-msisdn-submittoken)

use ruma_api::ruma_api;
use ruma_identifiers::{ClientSecretBox, SessionIdBox};

ruma_api! {
    metadata: {
        description: "Validate ownership of an email address.",
        method: GET,
        name: "validate_email_by_end_user",
        path: "/_matrix/identity/v2/validate/msisdn/submitToken",
        authentication: AccessToken,
        rate_limited: false,
    }

    request: {
        /// The session ID, generated by the `requestToken` call.
        #[ruma_api(query)]
        pub sid: &'a SessionIdBox,

        /// The client secret that was supplied to the `requestToken` call.
        #[ruma_api(query)]
        pub client_secret: &'a ClientSecretBox,

        /// The token generated by the `requestToken` call and sent to the user.
        #[ruma_api(query)]
        pub token: &'a str,
    }

    #[derive(Default)]
    response: {}
}

impl<'a> Request<'a> {
    /// Create a new `Request` with the given session ID, client secret and token.
    pub fn new(sid: &'a SessionIdBox, client_secret: &'a ClientSecretBox, token: &'a str) -> Self {
        Self { sid, client_secret, token }
    }
}

impl Response {
    /// Create a new empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
