package one.tesseract.client

interface Delegate {
    fun selectTransport(transports: Map<String, String>) //second is status
}