package one.tesseract.ipc.service

import one.tesseract.transport.service.BoundTransport
import one.tesseract.transport.service.JavaTransport
import one.tesseract.transport.service.Processor

class IPCTransport(private val channel: String = "default"): JavaTransport {
    companion object {
        @JvmStatic
        fun default(): IPCTransport = IPCTransport()
    }

    override fun bind(processor: Processor): BoundTransport =
        IPCBoundTransport(Channel.create(channel, processor))
}