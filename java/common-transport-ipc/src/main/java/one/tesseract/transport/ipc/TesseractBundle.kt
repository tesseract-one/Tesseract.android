//===------------ TesseractBundle.kt --------------------------------------------===//
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

package one.tesseract.transport.ipc

import android.os.Bundle

//BundleConstants
private object BC {
    val ID = "id"
    val TX = "tx"
    val RX = "rx"
}

public var Bundle.id: String?
    get() = this.getString(BC.ID)
    set(value) = this.putString(BC.ID, value)

public var Bundle.rx: ByteArray?
    get() = this.getByteArray(BC.RX)
    set(value) = this.putByteArray(BC.RX, value)

public var Bundle.tx: ByteArray?
    get() = this.getByteArray(BC.TX)
    set(value) = this.putByteArray(BC.TX, value)

public fun Bundle.toEmptyResponse(): Bundle {
    var response = Bundle()
    response.id = this.id
    return response
}

public fun RequestBundle(id: String, tx: ByteArray) = Bundle().apply { this.id = id; this.tx = tx }
public fun ResponseBundle(id: String, rx: ByteArray?) = Bundle().apply { this.id = id; this.rx = rx }