package one.tesseract.client.transport

sealed interface Status {
    data object Ready: Status

    data class Unavailable(val reason: String): Status
    data class Error(val error: Exception): Status
}

