package one.tesseract.client.transport

sealed interface Status {
    object Ready

    data class Unavailable(val reason: String)
    data class Error(val error: Exception)
}

