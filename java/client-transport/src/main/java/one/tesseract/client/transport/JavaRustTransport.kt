package one.tesseract.client.transport

import one.tesseract.common.transport.RustTransport

class JavaRustTransport(private val transport: JavaTransport): RustTransport, ClientTransport {
    external override fun createApplicator(): Long
}