//! Types for the *m.dummy* event.

use std::fmt;

use ruma_events_macros::EventContent;
use serde::{
    de::{self, Deserialize, Deserializer},
    ser::{Serialize, SerializeStruct as _, Serializer},
};

/// The payload for `DummyEvent`.
///
/// This event type is used to indicate new Olm sessions for end-to-end encryption.
///
/// Typically it is encrypted as an *m.room.encrypted* event, then sent as a to-device event.
///
/// The event does not have any content associated with it. The sending client is expected to
/// send a key share request shortly after this message, causing the receiving client to process
/// this *m.dummy* event as the most recent event and using the keyshare request to set up the
/// session. The keyshare request and *m.dummy* combination should result in the original sending
/// client receiving keys over the newly established session.
#[derive(Clone, Debug, EventContent)]
#[ruma_event(type = "m.dummy", kind = ToDevice)]
pub struct DummyToDeviceEventContent;

impl<'de> Deserialize<'de> for DummyToDeviceEventContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DummyToDeviceEventContent;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                while map.next_entry::<de::IgnoredAny, de::IgnoredAny>()?.is_some() {}
                Ok(DummyToDeviceEventContent)
            }
        }

        deserializer.deserialize_struct("DummyToDeviceEventContent", &[], Visitor)
    }
}

impl Serialize for DummyToDeviceEventContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_struct("DummyToDeviceEventContent", 0)?.end()
    }
}
