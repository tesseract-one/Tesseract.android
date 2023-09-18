package one.tesseract.service.transport

import one.tesseract.service.RustTransport

class IPCTransport(@Suppress("unused") val channel: String = "default"): RustTransport {
    external override fun createApplicator(): Long

//    override fun createApplicator(): Long {
//        //val a = this.getChannel();
//        TODO()
//    }
}