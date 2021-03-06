// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use super::{ErrorCode, FfiResult};
use super::callback::{Callback, CallbackArgs};
use std::fmt::{Debug, Display};
use std::os::raw::c_void;
use std::panic::{self, AssertUnwindSafe};

fn catch_unwind_result<'a, F, T, E>(f: F) -> Result<T, E>
    where F: FnOnce() -> Result<T, E>,
          E: Debug + From<&'a str>
{
    match panic::catch_unwind(AssertUnwindSafe(f)) {
        Err(_) => Err(E::from("panic")),
        Ok(result) => result,
    }
}

/// Catch panics. On error return the error code.
pub fn catch_unwind_error_code<'a, F, E>(f: F) -> i32
    where F: FnOnce() -> Result<(), E>,
          E: Debug + ErrorCode + From<&'a str>
{
    ffi_result_code!(catch_unwind_result(f))
}

/// Catch panics. On error call the callback.
pub fn catch_unwind_cb<'a, U, C, F, E>(user_data: U, cb: C, f: F)
    where U: Into<*mut c_void>,
          C: Callback + Copy,
          F: FnOnce() -> Result<(), E>,
          E: Debug + Display + ErrorCode + From<&'a str>
{
    if let Err(err) = catch_unwind_result(f) {
        let (error_code, description) = ffi_result!(Err::<(), E>(err));
        cb.call(user_data.into(),
                FfiResult {
                    error_code,
                    description: description.as_ptr(),
                },
                CallbackArgs::default());
    }
}
