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

package one.tesseract.example.app

import android.os.Build
import android.util.Log
import androidx.annotation.RequiresApi
import one.tesseract.ipc.client.*

class RustCore(public val application: Application) {
    companion object {
        init {
            System.loadLibrary("app")
        }
    }

    val transceiver2: Transceiver = Transceiver(ActivityMonitor(application))
    var service: Long = 0
    var executor: Long = 0
    lateinit var clName: String

    init {
        clName = this.javaClass.classLoader.javaClass.canonicalName
        Log.v("TEST", clName)
        //System.getProperties().setProperty("java.system.class.loader", this.javaClass.classLoader.javaClass.canonicalName)
        rustInit(this.javaClass.classLoader)
    }

    fun initThreadCL() {
        System.setProperty("java.system.class.loader", clName)
    }

    private external fun rustInit(loader: ClassLoader)

    //To be rewritten in Rust
    @RequiresApi(Build.VERSION_CODES.S)
    fun makeTransactionJava() {
        transceiver2.transceive("TransactionToSign".toByteArray()).whenComplete { response, exception ->
            if (exception == null) {
                when (response) {
                    is TransceiverResponseOk -> {
                        Log.v("SUCCESS", "We got some: ${response.data}")
                    }
                    is TransceiverResponseCanceled -> {
                        Log.v("SUCCESS", "User canceled the request")
                    }
                    is TransceiverResponseError -> {
                        Log.v("SUCCESS", "Something is wrong: ${response.exception}")
                    }
                }
            } else {
                Log.v("ERROR", "Something is badly wrong: $exception")
            }
        }
    }

    external fun makeTransaction()
}