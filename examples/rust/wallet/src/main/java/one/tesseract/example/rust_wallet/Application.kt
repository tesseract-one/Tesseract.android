package one.tesseract.example.rust_wallet

class Application: android.app.Application() {
    lateinit var rustCore: RustCore

    override fun onCreate() {
        super.onCreate()

        rustCore = RustCore(this)
    }

    fun readSignature(): String {
        return rustCore.readSignature()
    }

    fun saveSignature(signature: String) {
        rustCore.saveSignature(signature)
    }
}