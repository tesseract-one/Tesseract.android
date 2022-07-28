package com.example.wallet

import one.tesseract.ipc.service.Channel
import one.tesseract.ipc.service.Processor
import java.util.concurrent.CompletableFuture
import java.util.concurrent.CompletionStage

class RustCore(public val application: Application) {
    val processor: Processor = object : Processor {
        override fun process(data: ByteArray): CompletionStage<ByteArray> {
            return CompletableFuture.completedFuture(
                "json{\"id\":1,\"response\":{\"status\":\"ok\",\"signed\":\"testTransaction_signed!\"}}".toByteArray()
            )
        }
    }

    init {

    }

    val channel: Channel = Channel.create("default") { data ->
        CompletableFuture.completedFuture(
            "json{\"id\":1,\"response\":{\"status\":\"ok\",\"signed\":\"testTransaction_signed!\"}}".toByteArray()
        )
    }

    companion object {
        init {
            System.loadLibrary("wallet")
        }
    }
}