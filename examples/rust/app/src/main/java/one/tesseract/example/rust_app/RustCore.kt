//===------------ RustCore.kt --------------------------------------------===//
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

package one.tesseract.example.rust_app

import java.util.concurrent.CompletionStage

class RustCore(public val application: Application) {
    companion object {
        init {
            System.loadLibrary("app")
        }
    }

    var service: Long = 0
    var executor: Long = 0

    init {
        this.javaClass.classLoader?.let { rustInit(it) }
    }

    private external fun rustInit(loader: ClassLoader)
    external fun sign(transaction: String): CompletionStage<String>
    external fun execute(future: CompletionStage<String>)//on thread pool
}