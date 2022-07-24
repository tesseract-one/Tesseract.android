//===------------ Transceiver.kt --------------------------------------------===//
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

package one.tesseract.ipc.client

import android.app.Activity
import android.app.Application
import android.content.Intent
import android.os.Build
import android.os.Bundle
import androidx.annotation.RequiresApi
import java.util.*
import java.util.concurrent.CompletableFuture
import java.util.concurrent.CompletionStage

sealed class TransceiverResponse
object TransceiverResponseCanceled: TransceiverResponse()
class TransceiverResponseOk(val data: ByteArray): TransceiverResponse()
class TransceiverResponseError(val exception: TransceiverException): TransceiverResponse()

object TransceiverRegistry {
    //<ID, <ACTIVITY_RESULT, DATA>>
    private val resolvers: HashMap<String, CompletableFuture<Pair<Int, Bundle>>> = HashMap()

    fun register(id: String, resolver: CompletableFuture<Pair<Int, Bundle>>) {
        synchronized(resolvers) {
            if (resolvers.put(id, resolver) != null) {
                throw RuntimeException("Why is the ID duplicate? Report the bug, please")
            }
        }
    }

    @RequiresApi(Build.VERSION_CODES.N)
    fun resolve(reply: Pair<Int, Bundle>) {
        val (_, data) = reply
        //TODO: make sure that Tesseract activity passes the id properly (along with an error) in case it's not passed by the wallet
        val id = data.getString("id")
            ?: throw RuntimeException("Malformed data: no ID. Please, report the bug")

        val resolver = synchronized(resolvers) {
            resolvers.remove(id) ?: throw RuntimeException("No resolver for ID. Please, report the bug")
        }

        //TODO: properly implement the logic including result code and error handling, probably not here
        resolver.complete(reply)
    }
}

class TransceiverException(message:String): Exception(message)

class Transceiver(val activityMonitor: ActivityMonitor) {
    public constructor(application: Application): this(ActivityMonitor(application)) {
    }

    private fun sendThroughActivity(bundle: Bundle) {
        val activity = activityMonitor.activity

        var intent = Intent(activity.applicationContext, TesseractActivity::class.java)
        intent.putExtras(bundle)

        activity.startActivity(intent)
    }

    @RequiresApi(Build.VERSION_CODES.S)
    fun transceive(data: ByteArray): CompletionStage<TransceiverResponse> {
        val id = UUID.randomUUID().toString()

        val bundle = RequestBundle(id, data)

        val resultWithCode = CompletableFuture<Pair<Int, Bundle>>()

        TransceiverRegistry.register(id, resultWithCode)

        sendThroughActivity(bundle)

        return resultWithCode.thenCompose { it ->
            val (code, bundle) = it
            when (code) {
                Activity.RESULT_OK -> {
                    val rx = bundle.rx
                    if (rx == null) {
                        CompletableFuture.failedStage<TransceiverResponse>(TransceiverException("No data"))
                    } else {
                        CompletableFuture.completedFuture(TransceiverResponseOk(rx))
                    }
                }
                Activity.RESULT_CANCELED -> {
                    CompletableFuture.completedFuture(TransceiverResponseCanceled)
                }
                else -> {
                    CompletableFuture.failedStage<TransceiverResponse>(TransceiverException("Unknown code"))
                }
            }
        }
    }
}