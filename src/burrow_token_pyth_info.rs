use crate::*;

#[derive(BorshSerialize, BorshDeserialize, BorshSchema, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BurrowTokenPythInfo {
    pub decimals: u8,
    pub fraction_digits: u8,
    pub price_identifier: PriceIdentifier,
}

#[derive(BorshDeserialize, BorshSerialize, BorshSchema, PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub struct PriceIdentifier(pub [u8; 32]);

#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct PythPrice {
    pub price: i64,
    /// Confidence interval around the price
    pub conf: u64,
    /// The exponent
    pub expo: i32,
    /// Unix timestamp of when this price was computed
    pub publish_time: i64,
}

impl<'de> near_sdk::serde::Deserialize<'de> for PriceIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: near_sdk::serde::Deserializer<'de>,
    {
        /// A visitor that deserializes a hex string into a 32 byte array.
        struct IdentifierVisitor;

        impl<'de> near_sdk::serde::de::Visitor<'de> for IdentifierVisitor {
            /// Target type for either a hex string or a 32 byte array.
            type Value = [u8; 32];

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a hex string")
            }

            // When given a string, attempt a standard hex decode.
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: near_sdk::serde::de::Error,
            {
                if value.len() != 64 {
                    return Err(E::custom(format!(
                        "expected a 64 character hex string, got {}",
                        value.len()
                    )));
                }
                let mut bytes = [0u8; 32];
                hex::decode_to_slice(value, &mut bytes).map_err(E::custom)?;
                Ok(bytes)
            }
        }

        deserializer
            .deserialize_any(IdentifierVisitor)
            .map(PriceIdentifier)
    }
}

impl near_sdk::serde::Serialize for PriceIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: near_sdk::serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

impl std::string::ToString for PriceIdentifier {
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}
