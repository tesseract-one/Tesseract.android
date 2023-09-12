package one.tesseract.service

open class Tesseract(@Suppress("unused") private val ptr: Long = 0) {
    companion object {
        init {
            System.loadLibrary("tesseracta")
        }
    }

    init {
        create()
    }

    private external fun create()

    @Synchronized
    external fun addService(service: Service)
    @Synchronized
    external fun addTransport(transport: Transport)

    protected external fun finalize()
}