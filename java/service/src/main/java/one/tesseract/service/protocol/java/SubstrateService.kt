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
        extrinsicData: Array<UByte>,
        extrinsicMetadata: Array<UByte>,
        extrinsicTypes: Array<UByte>): CompletionStage<Array<UByte>> //UserCancelledException
}