package one.tesseract.transport.service

class JavaRustTransport(private val transport: JavaTransport): RustTransport {
    external override fun createApplicator(): Long
}