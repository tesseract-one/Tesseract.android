package one.tesseract.ipc.service

import android.os.Bundle
import java.util.concurrent.CompletionStage

//import one.tesseract.ipc.*

typealias FnListen = (ByteArray) -> CompletionStage<ByteArray>

class Channel(val listen: FnListen) {
    companion object {
        var channels = HashMap<String, Channel>()

        fun byId(id: String): Channel? = synchronized(channels) { channels[id] }

        fun create(id: String, listen: FnListen): Channel {
            val channel = Channel(listen)
            synchronized(channels) {
                channels[id] = channel
            }
            return channel
        }

        fun send(id: String, data: ByteArray): CompletionStage<ByteArray>? {
            val channel = byId(id)
            return channel?.listen?.invoke(data)
        }
    }
}