package one.tesseract.example.rust_wallet

import one.tesseract.activity.ActivityMonitor
import one.tesseract.activity.free.Launcher
import java.util.concurrent.CompletionStage

//suppressing because of rust interop
@Suppress("unused", "CanBeParameter")
class RustCore(/*Keep as it's called in RustCore*/ private val application: Application) {
    private val launcher: Launcher = Launcher(ActivityMonitor(application))

    //Keep as it's called in RustCore
    var signatureProvider: Long = 0

    init {
        rustInit(application.applicationInfo.dataDir)
    }

    private external fun rustInit(dataDir: String)
    external fun saveSignature(signature: String)
    external fun readSignature(): String

    //Keep as it's called in RustCore
    private fun requestUserConfirmation(transaction: String): CompletionStage<Boolean> {
        return SignActivity.requestUserConfirmation(launcher, transaction)
    }

    companion object {
        init {
            System.loadLibrary("wallet")
        }
    }
}