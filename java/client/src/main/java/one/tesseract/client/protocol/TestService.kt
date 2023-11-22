package one.tesseract.client.protocol

import one.tesseract.crabdroid.RustObject
import java.util.concurrent.CompletionStage
import one.tesseract.protocol.java.TestService as JTestService

@Suppress("unused") //used from Rust
class TestService(handle: Long): RustObject(handle), JTestService {
    external override fun signTransaction(transaction: String): CompletionStage<String>
}