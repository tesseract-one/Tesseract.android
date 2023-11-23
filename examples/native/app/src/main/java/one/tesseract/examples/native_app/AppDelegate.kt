package one.tesseract.examples.native_app

import android.util.Log
import one.tesseract.client.kotlin.Delegate

class AppDelegate: Delegate {
    override suspend fun selectTransport(transports: Map<String, one.tesseract.client.transport.Status>): String? {
        Log.d("DELEGATE", "This is a delegate in java")

        return try {
            transports.keys.first()
        } catch (_: NoSuchElementException) {
            null
        }
    }
}