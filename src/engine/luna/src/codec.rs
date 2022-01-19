// Copyright 2021 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bytes::BufMut;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

pub type Timestamp = u64;
pub type Value = Option<Vec<u8>>;

#[repr(u8)]
enum ValueKind {
    Some = 0,
    None = 1,
    Unknown = 255,
}

impl From<ValueKind> for u8 {
    fn from(v: ValueKind) -> Self {
        v as u8
    }
}

impl From<u8> for ValueKind {
    fn from(v: u8) -> Self {
        match v {
            0 => ValueKind::Some,
            1 => ValueKind::None,
            _ => ValueKind::Unknown,
        }
    }
}

pub fn put_timestamp(buf: &mut impl BufMut, ts: Timestamp) {
    buf.put_u64(ts);
}

pub fn put_length_prefixed_slice(buf: &mut impl BufMut, s: &[u8]) {
    buf.put_u64(s.len() as u64);
    buf.put_slice(s);
}

pub fn put_value(buf: &mut impl BufMut, value: &Value) {
    match value {
        Some(value) => {
            buf.put_u8(ValueKind::Some.into());
            buf.put_slice(value);
        }
        None => {
            buf.put_u8(ValueKind::None.into());
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlushDesc {
    pub memtable_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UpdateDesc {
    Flush(FlushDesc),
}

impl UpdateDesc {
    pub fn encode_to_vec(&self) -> Vec<u8> {
        let buf = serde_json::to_string(self).unwrap();
        buf.into()
    }

    pub fn decode_from(buf: &[u8]) -> Result<Self> {
        let desc: Self =
            serde_json::from_slice(buf).map_err(|x| Error::Corrupted(x.to_string()))?;
        Ok(desc)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableDesc {
    pub table_size: u64,
}

impl TableDesc {
    pub fn encode_to_vec(&self) -> Vec<u8> {
        let buf = serde_json::to_string(self).unwrap();
        buf.into()
    }

    pub fn decode_from(buf: &[u8]) -> Result<Self> {
        let desc: Self =
            serde_json::from_slice(buf).map_err(|x| Error::Corrupted(x.to_string()))?;
        Ok(desc)
    }
}
