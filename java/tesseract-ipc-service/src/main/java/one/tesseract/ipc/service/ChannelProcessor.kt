package one.tesseract.ipc.service

import one.tesseract.transport.service.Processor

fun Processor.listener(): FnListen {
    return { data ->
        this.process(data)
    }
}

fun Channel.Companion.create(id: String, processor: Processor): Channel {
    return create(id, processor.listener())
}