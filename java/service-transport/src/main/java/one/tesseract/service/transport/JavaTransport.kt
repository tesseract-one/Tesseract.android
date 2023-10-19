package one.tesseract.service.transport

interface JavaTransport: Transport {
    fun bind(processor: Processor): BoundTransport

    override fun rustTransport(): RustTransport = JavaRustTransport(this)
}