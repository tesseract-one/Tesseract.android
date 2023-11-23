package one.tesseract.client

import android.util.Log
import one.tesseract.client.kotlin.Delegate
import one.tesseract.client.transport.Status

class DefaultDelegate: Delegate {
    override suspend fun selectTransport(transports: Map<String, Status>): String? {
        return try {
            transports.keys.first()
        } catch (_: NoSuchElementException) {
            null
        }
    }
}