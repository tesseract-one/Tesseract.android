package one.tesseract.client.transport.ipc

import android.content.Context
import android.content.Intent
import android.os.Bundle

fun Intent.convertToCall(): Intent {
    val type = this.type
        ?: throw RuntimeException("Invalid API usage of Tesseract Activity - empty type")
    val data = this.extras
        ?: throw RuntimeException("Invalid API usage of Tesseract Activity - empty data")

    return IntentFactory.callWithType(data, type)
}

class IntentFactory {
    companion object {
        const val CALL = "one.tesseract.CALL"

        fun type(protocol: String): String {
            return "tesseract/$protocol"
        }

        fun internal(context: Context, data: Bundle, protocol: String): Intent {
            val intent = Intent(context, TesseractActivity::class.java)

            intent.putExtras(data)
            intent.type = type(protocol)

            return intent
        }

        fun callWithType(data: Bundle?, type: String): Intent {
            val intent = Intent()

            intent.action = CALL
            intent.type = type

            data?.also {
                intent.putExtras(it)
            }

            return intent
        }

        fun callWithProtocol(data: Bundle?, protocol: String): Intent {
            return callWithType(data, type(protocol))
        }
    }
}