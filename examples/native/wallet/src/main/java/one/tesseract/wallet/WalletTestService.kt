package one.tesseract.wallet

import one.tesseract.UserCancelledException
import one.tesseract.ipc.activity.free.Launcher
import one.tesseract.service.protocol.kotlin.TestService

class WalletTestService(private val launcher: Launcher): TestService {
    override suspend fun signTransaction(transaction: String): String {
        val allow = SignActivity.requestUserConfirmation(launcher, transaction)
        return if(allow) {
            transaction + "_signe15"
        } else {
            throw UserCancelledException("user cancelled for sure")
        }
    }
}