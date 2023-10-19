package one.tesseract.service.transport

class JavaRustTransport(private val transport: JavaTransport): RustTransport {
    external override fun createApplicator(): Long
}