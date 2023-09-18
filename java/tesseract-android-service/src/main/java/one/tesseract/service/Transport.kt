package one.tesseract.service

interface Transport {
    fun rustTransport(): RustTransport
}

interface RustTransport: Transport {
    fun createApplicator(): Long

    override fun rustTransport(): RustTransport {
        return this
    }
}

interface JavaTransport: Transport {
    override fun rustTransport(): RustTransport = JavaRustTransport(this)
}

class JavaRustTransport(private val transport: JavaTransport): RustTransport {
    external override fun createApplicator(): Long
}
