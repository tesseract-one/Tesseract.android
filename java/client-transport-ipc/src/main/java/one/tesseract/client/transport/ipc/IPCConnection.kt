package one.tesseract.client.transport.ipc

import android.os.Build
import androidx.annotation.RequiresApi
import one.tesseract.exception.UserCancelledException
import java.util.LinkedList
import one.tesseract.client.transport.Connection as CConnection
import java.util.concurrent.CompletionStage

class IPCConnection(private val transceiver: Transceiver, private val protocol: String): CConnection {
    private val responses = LinkedList<CompletionStage<TransceiverResponse>>()

    @RequiresApi(Build.VERSION_CODES.S)
    override fun send(data: ByteArray): CompletionStage<Unit> {
        val response = transceiver.transceive(data, protocol)

        synchronized(responses) {
            responses.push(response)
        }

        return response.thenApplyAsync {
        }
    }

    override fun receive(): CompletionStage<ByteArray> {
        return synchronized(responses) {
            val response = responses.pollFirst() ?: throw Exception("Nothing to receive. Send first.")
            response.thenApplyAsync {
                when(it) {
                    is TransceiverResponseCanceled -> {
                        throw UserCancelledException()
                    }
                    is TransceiverResponseOk -> {
                        it.data
                    }
                }
            }
        }
    }
}