package one.tesseract.client.transport

import one.tesseract.common.transport.RustTransport
import one.tesseract.common.transport.Transport
import java.util.concurrent.CompletionStage

interface JavaTransport: ClientTransport {
    fun id(): String
    fun status(protocol: String): CompletionStage<Status>

    fun connect(protocol: String): Connection

    override fun rustTransport(): RustTransport = JavaRustTransport(this)
}