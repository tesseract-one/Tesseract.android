package one.tesseract.client.transport

import java.util.concurrent.CompletionStage

interface Connection {
    fun send(data: ByteArray): CompletionStage<Unit>
    fun receive(): CompletionStage<ByteArray>
}