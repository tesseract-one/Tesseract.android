package one.tesseract.service.transport

import one.tesseract.common.transport.RustTransport

class JavaRustTransport(private val transport: JavaTransport): RustTransport, ServiceTransport {
    external override fun createApplicator(): Long
}