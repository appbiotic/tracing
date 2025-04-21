use anyhow::Context;
use prost::Message;

include!(concat!(
    env!("OUT_DIR"),
    "/appbiotic_api_prost_serde_build/_index.rs"
));

#[cfg(feature = "uniffi")]
uniffi::custom_type!(Config, Vec<u8>, {
    lower: |s| s.encode_to_vec(),
    try_lift: |s| Config::decode(s.as_slice()).context("Failed to decode appbiotic.tracing.Config"),
});

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::prost_serde::{Config, Preset};

    #[test]
    fn it_compiles() {
        let value = json!({
            "preset": Preset::PlainText as i32,
        });
        let config: Config = serde_json::from_value(value).unwrap();
        assert_eq!(config.preset(), Preset::PlainText);
    }
}
