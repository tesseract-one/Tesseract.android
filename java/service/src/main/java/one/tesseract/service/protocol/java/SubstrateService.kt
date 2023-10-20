package one.tesseract.service.protocol.java

import one.tesseract.service.protocol.common.substrate.GetAccountResponse
import one.tesseract.service.protocol.common.substrate.AccountType
import one.tesseract.service.service.java.Service
import java.util.concurrent.CompletionStage

interface SubstrateService: Service {
    fun getAccount(type: AccountType): CompletionStage<GetAccountResponse> //UserCancelledException

    fun signTransaction(
        accountType: AccountType,
        accountPath: String,
        extrinsicData: ByteArray,
        extrinsicMetadata: ByteArray,
        extrinsicTypes: ByteArray): CompletionStage<ByteArray> //UserCancelledException
}