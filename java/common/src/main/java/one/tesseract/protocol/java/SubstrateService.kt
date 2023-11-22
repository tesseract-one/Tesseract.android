package one.tesseract.protocol.java

import java.util.concurrent.CompletionStage
import kotlinx.coroutines.future.asDeferred

import one.tesseract.protocol.common.substrate.GetAccountResponse
import one.tesseract.protocol.common.substrate.AccountType
import one.tesseract.service.java.Service

import one.tesseract.protocol.kotlin.SubstrateService as KotlinSubstrateService

interface SubstrateService: Service {
    fun getAccount(type: AccountType): CompletionStage<GetAccountResponse> //UserCancelledException

    fun signTransaction(
        accountType: AccountType,
        accountPath: String,
        extrinsicData: ByteArray,
        extrinsicMetadata: ByteArray,
        extrinsicTypes: ByteArray): CompletionStage<ByteArray> //UserCancelledException

    override fun toKotlin(): KotlinSubstrateService = SubstrateServiceAdapter(this)
}

class SubstrateServiceAdapter(private val internal: SubstrateService): KotlinSubstrateService {
    override suspend fun getAccount(type: AccountType): GetAccountResponse =
        internal.getAccount(type).asDeferred().await()
    override suspend fun signTransaction(
        accountType: AccountType,
        accountPath: String,
        extrinsicData: ByteArray,
        extrinsicMetadata: ByteArray,
        extrinsicTypes: ByteArray
    ): ByteArray =
        internal.signTransaction(
            accountType,
            accountPath,
            extrinsicData,
            extrinsicMetadata,
            extrinsicTypes).asDeferred().await()
}