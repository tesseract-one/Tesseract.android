package one.tesseract.service.transport

interface Transport {
    fun rustTransport(): RustTransport
}