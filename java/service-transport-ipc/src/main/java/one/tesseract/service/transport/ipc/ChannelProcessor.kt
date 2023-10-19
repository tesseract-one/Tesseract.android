package one.tesseract.service.transport.ipc

import one.tesseract.service.transport.Processor

fun Processor.listener(): FnListen {
    return { data ->
        this.process(data)
    }
}

fun Channel.Companion.create(id: String, processor: Processor): Channel {
    return create(id, processor.listener())
}