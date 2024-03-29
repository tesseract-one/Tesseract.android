package one.tesseract.example.native_wallet

import one.tesseract.exception.UserCancelledException
import one.tesseract.activity.detached.Launcher
import one.tesseract.protocol.kotlin.TestService

class WalletTestService(private val launcher: Launcher): TestService {
    override suspend fun signTransaction(transaction: String): String {
        val allow = SignActivity.requestUserConfirmation(launcher, transaction)
        return if(allow) {
            transaction + "_signe_new1"
        } else {
            //throw UserCancelledException("user cancelled for sure")
            throw UserCancelledException()
        }
    }
}