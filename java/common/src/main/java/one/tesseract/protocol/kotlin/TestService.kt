package one.tesseract.protocol.kotlin

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.future.future
import one.tesseract.exception.UserCancelledException
import java.util.concurrent.CompletionStage
import one.tesseract.service.kotlin.Service as KotlinService
import one.tesseract.service.java.Service as JavaService
import one.tesseract.protocol.java.TestService as JavaTestService

import kotlin.jvm.Throws

interface TestService: KotlinService {
    @Throws(UserCancelledException::class)
    suspend fun signTransaction(transaction: String): String

    override fun toJava(scope: CoroutineScope): JavaService = TestServiceAdapter(this, scope)
}

class TestServiceAdapter(private val internal: TestService, private val scope: CoroutineScope): JavaTestService {
    override fun signTransaction(transaction: String): CompletionStage<String> {
        return scope.future {
            internal.signTransaction(transaction)
        }
    }
}