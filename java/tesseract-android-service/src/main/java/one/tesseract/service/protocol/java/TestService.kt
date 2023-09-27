package one.tesseract.service.protocol.java

import one.tesseract.service.service.java.Service
import java.util.concurrent.CompletionStage

interface TestService: Service {
    //UserCancelledException
    fun signTransaction(transaction: String): CompletionStage<String>
}