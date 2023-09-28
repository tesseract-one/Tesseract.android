package one.tesseract.transport.service

interface Transport {
    fun rustTransport(): RustTransport
}