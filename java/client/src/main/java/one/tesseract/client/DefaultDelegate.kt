package one.tesseract.client

import one.tesseract.client.transport.Status

class DefaultDelegate: Delegate {
    override fun selectTransport(transports: Map<String, Status>): String? {
        return try {
            transports.keys.first()
        } catch (_: NoSuchElementException) {
            null
        }
    }
}