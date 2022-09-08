package one.tesseract.example.wallet

class Application: android.app.Application() {
    lateinit var rustCore: RustCore

    override fun onCreate() {
        super.onCreate()

        rustCore = RustCore(this)
    }

    fun saveSignature(signature: String) {
        rustCore.saveSignature(signature)
    }
}