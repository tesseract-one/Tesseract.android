package one.tesseract.example.native_wallet

import android.os.Bundle

private const val TRANSACTION = "transaction"

fun Bundle.withTransaction(transaction: String): Bundle {
    putString(TRANSACTION, transaction)
    return this
}

val Bundle.transaction: String
    get() =
        getString(TRANSACTION) ?: throw RuntimeException("No transaction in the extras")