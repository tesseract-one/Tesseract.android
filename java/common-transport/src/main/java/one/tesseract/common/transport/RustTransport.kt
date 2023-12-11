package one.tesseract.common.transport

interface RustTransport: Transport {
    fun createApplicator(): Long

    override fun rustTransport(): RustTransport {
        return this
    }
}