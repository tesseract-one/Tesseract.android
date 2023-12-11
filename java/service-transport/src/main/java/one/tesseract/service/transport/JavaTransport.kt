package one.tesseract.service.transport

import one.tesseract.common.transport.RustTransport

interface JavaTransport: ServiceTransport {
    fun bind(processor: Processor): BoundTransport

    override fun rustTransport(): RustTransport = JavaRustTransport(this)
}