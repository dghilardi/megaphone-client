use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all="camelCase")]
pub struct ChannelCreateReqDto {
    /// Protocols supported by the client
    /// sorted by preference (first is the preferred)
    #[serde(default)]
    pub protocols: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ChannelCreateResDto {
    #[deprecated(since = "0.10.0", note = "use producer_address and consumer_address instead")]
    #[serde(default)]
    pub channel_id: String,
    pub producer_address: String,
    pub consumer_address: String,
    #[deprecated(since = "0.10.0", note = "agent name is embedded in producer_address and consumer_address")]
    #[serde(default)]
    pub agent_name: String,
    /// Protocols supported by the channel
    #[serde(default)]
    pub protocols: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteBatchReqDto {
    #[serde(alias = "channelIds")]
    pub channels: HashSet<String>,
    pub messages: Vec<ChanMessage>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChanMessage {
    pub stream_id: String,
    pub body: serde_json::Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteBatchResDto {
    pub failures: Vec<MessageDeliveryFailure>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDeliveryFailure {
    pub channel: String,
    pub index: usize,
    pub reason: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChanExistsReqDto {
    #[serde(alias = "channelIds")]
    pub channels: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChanExistsResDto {
    pub channels: HashMap<String, bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsListParams {
    #[serde(default)]
    pub agents: HashSet<String>,
    #[serde(default)]
    pub skip: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    50
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelInfoDto {
    pub address: String,
    pub agent_id: String,
}

impl FromStr for ChannelInfoDto {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            address: String::from(s),
            agent_id: s.split('.')
                .next()
                .map(ToString::to_string)
                .ok_or_else(|| anyhow!("Cannot extract agent from {s}"))?,
        })
    }
}