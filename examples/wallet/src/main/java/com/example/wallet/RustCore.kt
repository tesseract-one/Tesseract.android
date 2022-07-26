package com.example.wallet

import one.tesseract.ipc.service.Channel
import java.util.concurrent.CompletableFuture

class RustCore(public val application: Application) {
    val channel: Channel = Channel.create("default") { data ->
        CompletableFuture.completedFuture(
        "json{\"id\":1,\"response\":{\"status\":\"ok\",\"signed\":\"testTransaction_signed!\"}}".toByteArray())
    }

    companion object {
        init {
            System.loadLibrary("wallet")
        }
    }
}