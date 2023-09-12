package one.tesseract.service.protocol

import one.tesseract.UserCancelledException
import one.tesseract.service.Service
import java.util.concurrent.CompletionStage

interface TestService: Service {
    //UserCancelledException
    fun signTransaction(transaction: String): CompletionStage<String>
}