package one.tesseract.client.transport.ipc

import android.app.Application
import one.tesseract.client.transport.ClientTransport
import one.tesseract.common.transport.RustTransport

class IPCTransportR(val application: Application): RustTransport, ClientTransport {
    external override fun createApplicator(): Long
}