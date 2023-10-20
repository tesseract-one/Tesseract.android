package one.tesseract.example.native_wallet

import one.tesseract.activity.detached.Launcher
import one.tesseract.service.Tesseract

class Application(@Suppress("unused") private var tesseract: Tesseract? = null): android.app.Application() {
    override fun onCreate() {
        super.onCreate()

        val launcher = Launcher(this)

        this.tesseract = Tesseract
            .default()
            .service(WalletTestService(launcher))
    }
}