package one.tesseract.example.wallet

import android.os.Bundle
import one.tesseract.ipc.activity.ActivityMonitor
import one.tesseract.ipc.activity.free.Launcher
import java.util.concurrent.CompletionStage

class RustCore(public val application: Application) {
//    val processor: Processor = object : Processor {
//        override fun process(data: ByteArray): CompletionStage<ByteArray> {
//            return CompletableFuture.completedFuture(
//                "json{\"id\":1,\"response\":{\"status\":\"ok\",\"signed\":\"testTransaction_signed!\"}}".toByteArray()
//            )
//        }
//    }

    private val launcher: Launcher = Launcher(ActivityMonitor(application))

    var signatureProvider: Long = 0

    init {
        rustInit(application.applicationInfo.dataDir)
    }

    private external fun rustInit(dataDir: String)
    external fun saveSignature(signature: String)
    external fun readSignature(): String

//    val channel: Channel = Channel.create("default") { data ->
//        CompletableFuture.completedFuture(
//            "json{\"id\":1,\"response\":{\"status\":\"ok\",\"signed\":\"testTransaction_signed!\"}}".toByteArray()
//        )
//    }

    fun requestUserConfirmation(transaction: String): CompletionStage<Boolean> {
        return SignActivity.requestUserConfirmation(launcher, transaction)
    }

    companion object {
        init {
            System.loadLibrary("wallet")
        }
    }
}