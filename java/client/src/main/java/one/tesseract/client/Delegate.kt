package one.tesseract.client

import one.tesseract.client.transport.Status

interface Delegate {
    fun selectTransport(transports: Map<String, Status>): String?
}