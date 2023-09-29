package one.tesseract.wallet

import one.tesseract.ipc.activity.ActivityMonitor
import one.tesseract.ipc.activity.free.Launcher
import one.tesseract.service.Tesseract

class Application(@Suppress("unused") private var tesseract: Tesseract? = null): android.app.Application() {
    override fun onCreate() {
        super.onCreate()

        val launcher = Launcher(ActivityMonitor(this))

        this.tesseract = Tesseract
            .default()
            .service(WalletTestService(launcher))
    }
}