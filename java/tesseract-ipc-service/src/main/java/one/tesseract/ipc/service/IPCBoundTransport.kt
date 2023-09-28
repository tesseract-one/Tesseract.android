package one.tesseract.ipc.service

import one.tesseract.transport.service.BoundTransport

class IPCBoundTransport(private val channel: Channel): BoundTransport {
}