// Copyright 2015-2020 Capital One Services, LLC
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

//! # Errors
//!
//! This module contains types and utility functions for error handling

use tea_codec::error::code::wascc::*;
use tea_codec::error::{new_common_error_code, CommonCode, TeaError};

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

pub fn new(kind: ErrorKind) -> Error {
	Error(Box::new(kind))
}

#[derive(Debug)]
pub enum ErrorKind {
	UTF8(std::string::FromUtf8Error),
	UTF8Str(std::str::Utf8Error),
	HostError(String),
	BadDispatch(String),
}

impl Error {
	pub fn kind(&self) -> &ErrorKind {
		&self.0
	}

	pub fn into_kind(self) -> ErrorKind {
		*self.0
	}
}

impl Into<TeaError> for Error {
	fn into(self) -> TeaError {
		match *self.0 {
			ErrorKind::UTF8(e) => new_common_error_code(CommonCode::UTF8EncodingError)
				.to_error_code(Some(format!("{:?}", e)), None),
			ErrorKind::UTF8Str(e) => new_common_error_code(CommonCode::Utf8StrEncodingError)
				.to_error_code(Some(format!("{:?}", e)), None),
			ErrorKind::HostError(s) => {
				new_wascc_error_code(WasccCode::GeneralHostError).to_error_code(Some(s), None)
			}
			ErrorKind::BadDispatch(s) => {
				new_wascc_error_code(WasccCode::BadDispatch).to_error_code(Some(s), None)
			}
		}
	}
}

impl From<std::str::Utf8Error> for Error {
	fn from(source: std::str::Utf8Error) -> Error {
		Error(Box::new(ErrorKind::UTF8Str(source)))
	}
}

impl From<std::string::FromUtf8Error> for Error {
	fn from(source: std::string::FromUtf8Error) -> Error {
		Error(Box::new(ErrorKind::UTF8(source)))
	}
}
