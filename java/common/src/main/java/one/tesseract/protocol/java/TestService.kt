package one.tesseract.protocol.java

import one.tesseract.service.java.Service
import java.util.concurrent.CompletionStage
import kotlinx.coroutines.future.asDeferred

import one.tesseract.protocol.kotlin.TestService as KotlinTestService

interface TestService: Service {
    //UserCancelledException
    fun signTransaction(transaction: String): CompletionStage<String>

    override fun toKotlin(): KotlinTestService = TestServiceAdapter(this)
}

class TestServiceAdapter(private val internal: TestService): KotlinTestService {
    override suspend fun signTransaction(transaction: String): String =
        internal.signTransaction(transaction).asDeferred().await()
}