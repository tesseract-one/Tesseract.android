package one.tesseract.service

interface Transport {
    fun rustTransport(): RustTransport
}

class RustTransport: Transport {
    override fun rustTransport(): RustTransport {
        return this
    }
}
interface JavaTransport: Transport {
    override fun rustTransport(): RustTransport {
        return RustTransport() //do the actual wrapping here
    }
}
