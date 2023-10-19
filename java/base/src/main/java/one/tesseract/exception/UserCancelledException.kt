package one.tesseract.exception

open class UserCancelledException(message: String?, cause: Throwable?) : Exception(message, cause) {
    constructor() : this(null, null) {
    }

    constructor(message: String) : this(message, null) {
    }
}