package one.tesseract.client.protocol

import one.tesseract.crabdroid.RustObject
import one.tesseract.protocol.common.substrate.AccountType
import one.tesseract.protocol.common.substrate.GetAccountResponse
import java.util.concurrent.CompletionStage
import one.tesseract.protocol.java.SubstrateService as JSubstrateService

@Suppress("unused") //used from Rust
class SubstrateService(handle: Long): RustObject(handle), JSubstrateService {
    external override fun getAccount(type: AccountType): CompletionStage<GetAccountResponse>

    external override fun signTransaction(
        accountType: AccountType,
        accountPath: String,
        extrinsicData: ByteArray,
        extrinsicMetadata: ByteArray,
        extrinsicTypes: ByteArray
    ): CompletionStage<ByteArray>
}