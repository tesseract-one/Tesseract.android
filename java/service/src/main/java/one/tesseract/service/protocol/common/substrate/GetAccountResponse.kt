package one.tesseract.service.protocol.common.substrate

    data class GetAccountResponse(val publicKey: ByteArray, val path: String) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as GetAccountResponse

        if (!publicKey.contentEquals(other.publicKey)) return false
        if (path != other.path) return false

        return true
    }

    override fun hashCode(): Int {
        var result = publicKey.contentHashCode()
        result = 31 * result + path.hashCode()
        return result
    }
}