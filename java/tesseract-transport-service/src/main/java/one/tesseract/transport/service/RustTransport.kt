package one.tesseract.transport.service

interface RustTransport: Transport {
    fun createApplicator(): Long

    override fun rustTransport(): RustTransport {
        return this
    }
}