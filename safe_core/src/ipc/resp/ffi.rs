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

#![allow(unsafe_code)]

use routing::XOR_NAME_LEN;
use rust_sodium::crypto::{box_, secretbox, sign};

/// It represents the authentication response.
#[repr(C)]
#[derive(Clone)]
pub struct AuthGranted {
    /// The access keys.
    pub app_keys: AppKeys,
    /// Access container
    pub access_container: AccessContInfo,

    /// Crust's bootstrap config
    pub bootstrap_config_ptr: *mut u8,
    /// `bootstrap_config`'s length
    pub bootstrap_config_len: usize,
    /// Used by Rust memory allocator
    pub bootstrap_config_cap: usize,
}

impl Drop for AuthGranted {
    fn drop(&mut self) {
        unsafe {
            let _ = Vec::from_raw_parts(self.bootstrap_config_ptr,
                                        self.bootstrap_config_len,
                                        self.bootstrap_config_cap);
        }
    }
}

/// Represents the needed keys to work with the data
#[repr(C)]
#[derive(Copy)]
pub struct AppKeys {
    /// Owner signing public key
    pub owner_key: [u8; sign::PUBLICKEYBYTES],
    /// Data symmetric encryption key
    pub enc_key: [u8; secretbox::KEYBYTES],
    /// Asymmetric sign public key.
    ///
    /// This is the identity of the App in the Network.
    pub sign_pk: [u8; sign::PUBLICKEYBYTES],
    /// Asymmetric sign private key.
    pub sign_sk: [u8; sign::SECRETKEYBYTES],
    /// Asymmetric enc public key.
    pub enc_pk: [u8; box_::PUBLICKEYBYTES],
    /// Asymmetric enc private key.
    pub enc_sk: [u8; box_::SECRETKEYBYTES],
}

impl Clone for AppKeys {
    // Implemented manually because:
    //
    // error[E0277]: the trait bound `[u8; 64]: std::clone::Clone` is not satisfied
    //
    // There is a default implementation only until size 32
    fn clone(&self) -> Self {
        let mut sign_pk = [0; sign::PUBLICKEYBYTES];
        let mut sign_sk = [0; sign::SECRETKEYBYTES];

        sign_pk.copy_from_slice(&self.sign_pk);
        sign_sk.copy_from_slice(&self.sign_sk);

        AppKeys {
            owner_key: self.owner_key,
            enc_key: self.enc_key,
            sign_pk: sign_pk,
            sign_sk: sign_sk,
            enc_pk: self.enc_pk,
            enc_sk: self.enc_sk,
        }
    }
}

/// Access container
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AccessContInfo {
    /// ID
    pub id: [u8; XOR_NAME_LEN],
    /// Type tag
    pub tag: u64,
    /// Nonce
    pub nonce: [u8; secretbox::NONCEBYTES],
}
