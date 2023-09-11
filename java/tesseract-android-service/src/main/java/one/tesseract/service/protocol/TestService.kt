package one.tesseract.service.protocol

import one.tesseract.UserCancelledException
import one.tesseract.service.Service

interface TestService: Service {
    @Throws(UserCancelledException::class)
    fun signTransaction(transaction: String): String
}