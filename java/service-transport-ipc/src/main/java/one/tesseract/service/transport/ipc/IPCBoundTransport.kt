package one.tesseract.service.transport.ipc

import one.tesseract.service.transport.BoundTransport

class IPCBoundTransport(private val channel: Channel): BoundTransport {
}