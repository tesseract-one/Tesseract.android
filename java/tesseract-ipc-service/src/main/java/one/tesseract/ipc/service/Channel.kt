package one.tesseract.ipc.service

import android.os.Bundle
import java.util.*
import java.util.concurrent.CompletionStage

//import one.tesseract.ipc.*

typealias FnListen = (ByteArray) -> CompletionStage<ByteArray>

//TODO: make private later
public class Channel private constructor(val id: String) {
    companion object {
        public const val DEFAULT = "default"

        var channels = WeakHashMap<Channel, FnListen>()

        private fun byId(id: String): FnListen? = synchronized(channels) { channels[Channel(id)] }

        //create one here and keep until needed. It's going to be freed once gced
        public fun create(id: String, listen: FnListen): Channel {
            val channel = Channel(id)
            synchronized(channels) {
                if (channels.containsKey(channel)) {
                    throw IllegalArgumentException("Channel '$id' already exists")
                } else {
                    channels[channel] = listen
                }
            }
            return channel
        }

        //send data to the channel
        public fun send(id: String, data: ByteArray): CompletionStage<ByteArray>? {
            val listen = byId(id)
            return listen?.invoke(data)
        }
    }

    override fun equals(other: Any?): Boolean {
        return other?.let { it as Channel }?.let { it.id == id } ?: false
    }

    override fun hashCode(): Int {
        return id.hashCode()
    }

    override fun toString(): String {
        return "Tesseract IPC Internal Channel: $id"
    }
}