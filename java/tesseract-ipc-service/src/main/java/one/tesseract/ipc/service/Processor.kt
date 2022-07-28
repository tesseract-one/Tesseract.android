package one.tesseract.ipc.service

import java.util.concurrent.CompletionStage

public interface Processor {
    fun process(data: ByteArray): CompletionStage<ByteArray>
}

fun Processor.listener(): FnListen {
    return { data ->
        this.process(data)
    }
}

public fun Channel.Companion.create(id: String, processor: Processor): Channel {
    return create(id, processor.listener())
}

//TODO: do finalize
public class TransportProcessor(val native: Long) : Processor {
    external override fun process(data: ByteArray): CompletionStage<ByteArray>

    public fun createChannel(id: String): Channel = Channel.Companion.create(id, this)
}