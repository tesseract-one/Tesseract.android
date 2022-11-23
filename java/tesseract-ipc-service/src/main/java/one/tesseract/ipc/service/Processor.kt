package one.tesseract.ipc.service

import java.util.concurrent.CompletionStage

interface Processor {
    fun process(data: ByteArray): CompletionStage<ByteArray>
}

fun Processor.listener(): FnListen {
    return { data ->
        this.process(data)
    }
}

fun Channel.Companion.create(id: String, processor: Processor): Channel {
    return create(id, processor.listener())
}

//TODO: do finalize
class TransportProcessor(val native: Long) : Processor {
    external override fun process(data: ByteArray): CompletionStage<ByteArray>

    @Suppress("unused") //used from Rust
    fun createChannel(id: String): Channel = Channel.Companion.create(id, this)
}