pub mod some_endpoint {
    use ruma_api::{ruma_api, Outgoing};
    use ruma_events::{collections::all, sticker::StickerEvent, tag::TagEvent, EventResult};
    use serde::Serialize;

    ruma_api! {
        metadata {
            description: "Does something.",
            method: POST, // An `http::Method` constant. No imports required.
            name: "some_endpoint",
            path: "/_matrix/some/endpoint/:baz",
            rate_limited: false,
            requires_authentication: false,
        }

        request {
            // With no attribute on the field, it will be put into the body of the request.
            pub foo: String,

            // This value will be put into the "Content-Type" HTTP header.
            #[ruma_api(header = CONTENT_TYPE)]
            pub content_type: String,

            // This value will be put into the query string of the request's URL.
            #[ruma_api(query)]
            pub bar: String,

            // This value will be inserted into the request's URL in place of the
            // ":baz" path component.
            #[ruma_api(path)]
            pub baz: String,
        }

        response {
            // This value will be extracted from the "Content-Type" HTTP header.
            #[ruma_api(header = CONTENT_TYPE)]
            pub content_type: String,

            // With no attribute on the field, it will be extracted from the body of the response.
            pub value: String,

            // You can use serde attributes on any kind of field
            #[serde(skip_serializing_if = "Option::is_none")]
            pub optional_flag: Option<bool>,

            // This is how you usually use `#[wrap_incoming]` with event types
            #[wrap_incoming(with EventResult)]
            pub event: TagEvent,

            // Same for lists of events
            #[wrap_incoming(all::RoomEvent with EventResult)]
            pub list_of_events: Vec<all::RoomEvent>,

            // This is how `#[wrap_incoming]` is used with nested `EventResult`s
            #[wrap_incoming]
            pub object: ObjectContainingEvents,
        }
    }

    #[derive(Clone, Debug, Serialize, Outgoing)]
    pub struct ObjectContainingEvents {
        #[wrap_incoming(TagEvent with EventResult)]
        pub event_list_1: Vec<TagEvent>,
        #[wrap_incoming(StickerEvent with EventResult)]
        pub event_list_2: Vec<StickerEvent>,
    }
}

pub mod newtype_body_endpoint {
    use ruma_api_macros::ruma_api;

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct MyCustomType {
        pub foo: String,
    }

    ruma_api! {
        metadata {
            description: "Does something.",
            method: PUT,
            name: "newtype_body_endpoint",
            path: "/_matrix/some/newtype/body/endpoint",
            rate_limited: false,
            requires_authentication: false,
        }

        request {
            #[ruma_api(body)]
            pub file: Vec<u8>,
        }

        response {
            #[ruma_api(body)]
            pub my_custom_type: MyCustomType,
        }
    }
}

pub mod query_map_endpoint {
    use ruma_api_macros::ruma_api;

    ruma_api! {
        metadata {
            description: "Does something.",
            method: GET,
            name: "newtype_body_endpoint",
            path: "/_matrix/some/query/map/endpoint",
            rate_limited: false,
            requires_authentication: false,
        }

        request {
            #[ruma_api(query_map)]
            pub fields: Vec<(String, String)>,
        }

        response {
        }
    }
}
