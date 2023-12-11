package one.tesseract.client.transport.ipc

import android.app.Application
import one.tesseract.client.transport.Connection
import one.tesseract.client.transport.JavaTransport
import one.tesseract.client.transport.Status
import java.util.concurrent.CompletableFuture
import java.util.concurrent.CompletionStage

class IPCTransport(application: Application): JavaTransport {
    private val transceiver: Transceiver
    init {
        transceiver = Transceiver(application)
    }
    override fun id(): String = "IPC"

    override fun status(protocol: String): CompletionStage<Status> {
        val available = transceiver.ping(protocol)

        val status = if(available) {
            Status.Ready
        } else {
            Status.Unavailable("No wallet implementing '$protocol' protocol found installed on your device")
        }

        val future = CompletableFuture<Status>()
        future.complete(status)

        return future
    }

    override fun connect(protocol: String): Connection = IPCConnection(transceiver, protocol)
}