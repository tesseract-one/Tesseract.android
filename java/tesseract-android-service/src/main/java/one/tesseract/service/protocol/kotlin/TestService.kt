package one.tesseract.service.protocol.kotlin

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.future.future
import one.tesseract.UserCancelledException
import java.util.concurrent.CompletionStage
import one.tesseract.service.service.kotlin.Service as KotlinService
import one.tesseract.service.service.java.Service as JavaService
import one.tesseract.service.protocol.java.TestService as JavaTestService

import kotlin.jvm.Throws

interface TestService: KotlinService {
    @Throws(one.tesseract.UserCancelledException::class)
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