package one.tesseract.client.kotlin

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.future.future
import one.tesseract.client.transport.Status
import java.util.concurrent.Future

import one.tesseract.client.java.Delegate as JDelegate

interface Delegate {
    suspend fun selectTransport(transports: Map<String, Status>): String?
}

class DelegateAdapter(private val delegate: Delegate, private val scope: CoroutineScope): JDelegate {
    override fun selectTransport(transports: Map<String, Status>): Future<String?> = scope.future {
        delegate.selectTransport(transports)
    }
}