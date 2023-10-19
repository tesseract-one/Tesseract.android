package one.tesseract.service.transport.ipc

import one.tesseract.service.transport.BoundTransport
import one.tesseract.service.transport.JavaTransport
import one.tesseract.service.transport.Processor

class IPCTransport(private val channel: String = "default"): JavaTransport {
    companion object {
        @JvmStatic
        fun default(): IPCTransport = IPCTransport()
    }

    override fun bind(processor: Processor): BoundTransport =
        IPCBoundTransport(Channel.create(channel, processor))
}