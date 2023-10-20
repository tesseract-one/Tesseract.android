package one.tesseract.service.protocol.kotlin

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.future.future
import one.tesseract.exception.UserCancelledException
import one.tesseract.service.protocol.common.substrate.AccountType
import one.tesseract.service.protocol.common.substrate.GetAccountResponse
import java.util.concurrent.CompletionStage
import kotlin.jvm.Throws
import one.tesseract.service.service.kotlin.Service as KotlinService
import one.tesseract.service.service.java.Service as JavaService
import one.tesseract.service.protocol.java.SubstrateService as JavaSubstrateService

interface SubstrateService: KotlinService {
    @Throws(UserCancelledException::class)
    suspend fun getAccount(type: AccountType): GetAccountResponse

    @Throws(UserCancelledException::class)
    suspend fun signTransaction(
        accountType: AccountType,
        accountPath: String,
        extrinsicData: ByteArray,
        extrinsicMetadata: ByteArray,
        extrinsicTypes: ByteArray): ByteArray

    override fun toJava(scope: CoroutineScope): JavaService = SubstrateServiceAdapter(this, scope)
}

class SubstrateServiceAdapter(private val internal: SubstrateService, private val scope: CoroutineScope): JavaSubstrateService {
    override fun getAccount(type: AccountType): CompletionStage<GetAccountResponse> {
        return scope.future {
            internal.getAccount(type)
        }
    }

    override fun signTransaction(
        accountType: AccountType,
        accountPath: String,
        extrinsicData: ByteArray,
        extrinsicMetadata: ByteArray,
        extrinsicTypes: ByteArray
    ): CompletionStage<ByteArray> {
        return scope.future {
            internal.signTransaction(accountType, accountPath, extrinsicData, extrinsicMetadata, extrinsicTypes)
        }
    }
}