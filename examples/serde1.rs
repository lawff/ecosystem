use std::{fmt, str::FromStr};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    name: String,
    #[serde(rename = "privateAge")]
    age: u32,
    date_of_birth: DateTime<Local>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    skills: Vec<String>,
    state: WakeState,
    #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
    data: Vec<u8>,
    #[serde_as(as = "DisplayFromStr")]
    sensitive: SensitiveData,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    url: Vec<http::Uri>,
}

#[derive(Debug)]
struct SensitiveData(String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "details")]
enum WakeState {
    Working(String),
    OnLeave(DateTime<Local>),
    Terminated,
}

fn main() -> anyhow::Result<()> {
    let state = WakeState::OnLeave(Local::now());
    let user = User {
        name: "Alice".to_string(),
        state,
        data: vec![1, 2, 3, 4, 5],
        age: 30,
        date_of_birth: Local::now(),
        skills: vec!["Rust".to_string(), "Python".to_string()],
        url: vec!["https://example.com".parse()?],
        sensitive: SensitiveData::new("sensitive data"),
    };

    let json = serde_json::to_string(&user)?;
    println!("{}", json);

    let user1: User = serde_json::from_str(&json)?;
    println!("{:?}", user1);

    Ok(())
}

fn b64_encode<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encoded = URL_SAFE_NO_PAD.encode(data);
    serializer.serialize_str(&encoded)
}

fn b64_decode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded = String::deserialize(deserializer)?;
    let decoded = URL_SAFE_NO_PAD
        .decode(encoded.as_bytes())
        .map_err(serde::de::Error::custom)?;
    Ok(decoded)
}

impl fmt::Display for SensitiveData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encrypted = URL_SAFE_NO_PAD.encode(self.0.as_bytes());
        write!(f, "{}", encrypted)
    }
}

impl FromStr for SensitiveData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decrypted = URL_SAFE_NO_PAD.decode(s.as_bytes())?;
        Ok(Self(String::from_utf8(decrypted)?))
    }
}

impl SensitiveData {
    fn new(data: impl Into<String>) -> Self {
        Self(data.into())
    }
}
