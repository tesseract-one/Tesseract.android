package one.tesseract.common.transport

interface Transport {
    fun rustTransport(): RustTransport
}