//===------------ signature_provider.rs --------------------------------------------===//
//  Copyright 2022, Tesseract Systems, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//===----------------------------------------------------------------------===//

use std::fs;
use std::io;

pub (crate) struct SignatureProvider {
    data_dir: String
}

impl SignatureProvider {
    pub (crate) fn new(data_dir: &str) -> Self {
        Self { data_dir: data_dir.to_owned() }
    }

    fn file(&self) -> String {
        format!("{}/signature", &self.data_dir)
    }

    pub (crate) fn set_signature(&self, signature: &str) -> io::Result<()> {
        let file = self.file();
        fs::write(&file, signature)
    }

    pub (crate) fn get_signature(&self) -> String {
        let file = self.file();
        fs::read_to_string(&file).unwrap_or_else(|e| {
            debug!("The signature file is not found: {}", e);
            "_signed_default".to_owned()
        })
    }
}