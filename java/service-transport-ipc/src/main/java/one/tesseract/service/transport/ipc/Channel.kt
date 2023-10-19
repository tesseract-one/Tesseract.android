package one.tesseract.service.transport.ipc

import java.util.*
import java.util.concurrent.CompletionStage

typealias FnListen = (ByteArray) -> CompletionStage<ByteArray>

class Channel private constructor(private val id: String) {
    companion object {
        @Suppress("unused")
        private const val DEFAULT = "default"

        private var channels = WeakHashMap<Channel, FnListen>()

        private fun byId(id: String): FnListen? = synchronized(channels) { channels[Channel(id)] }

        //create one here and keep until needed. It's going to be freed once garbage collected
        fun create(id: String, listen: FnListen): Channel {
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
        fun send(id: String, data: ByteArray): CompletionStage<ByteArray>? {
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