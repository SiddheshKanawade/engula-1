// Copyright 2022 The Engula Authors.
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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0} is not found")]
    NotFound(String),
    #[error("{0} already exists")]
    AlreadyExists(String),
    #[error("{0}")]
    InvalidArgument(String),
    #[error("{0}")]
    Corrupted(String),
    #[error("{0}")]
    Internal(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Unknown(Box<dyn std::error::Error + Send>),
}

impl Error {
    pub fn invalid_argument(m: impl Into<String>) -> Self {
        Self::InvalidArgument(m.into())
    }

    pub fn corrupted(m: impl Into<String>) -> Self {
        Self::Corrupted(m.into())
    }

    pub fn internal(m: impl Into<String>) -> Self {
        Self::Internal(m.into())
    }

    pub fn unknown(err: impl std::error::Error + Send + 'static) -> Self {
        Self::Unknown(Box::new(err))
    }
}

impl From<tonic::Status> for Error {
    fn from(s: tonic::Status) -> Self {
        match s.code() {
            tonic::Code::NotFound => Error::NotFound(s.message().into()),
            tonic::Code::AlreadyExists => Error::AlreadyExists(s.message().into()),
            tonic::Code::InvalidArgument => Error::InvalidArgument(s.message().into()),
            tonic::Code::DataLoss => Error::Corrupted(s.message().into()),
            tonic::Code::Internal => Error::Internal(s.message().into()),
            _ => Error::Unknown(Box::new(s)),
        }
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(e: tonic::transport::Error) -> Self {
        Error::Unknown(Box::new(e))
    }
}

impl From<Error> for tonic::Status {
    fn from(err: Error) -> Self {
        let (code, message) = match err {
            Error::NotFound(s) => (tonic::Code::NotFound, s),
            Error::AlreadyExists(s) => (tonic::Code::AlreadyExists, s),
            Error::InvalidArgument(s) => (tonic::Code::InvalidArgument, s),
            Error::Corrupted(s) => (tonic::Code::DataLoss, s),
            Error::Internal(s) => (tonic::Code::Internal, s),
            Error::Io(s) => (tonic::Code::Unknown, s.to_string()),
            Error::Unknown(s) => (tonic::Code::Unknown, s.to_string()),
        };
        tonic::Status::new(code, message)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
