package one.tesseract.transport.service

interface JavaTransport: Transport {
    fun bind(processor: Processor): BoundTransport

    override fun rustTransport(): RustTransport = JavaRustTransport(this)
}