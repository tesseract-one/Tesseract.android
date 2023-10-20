package one.tesseract.activity.free

import android.os.Build
import androidx.annotation.RequiresApi
import java.util.*
import java.util.concurrent.CompletableFuture
import java.util.concurrent.CompletionStage

object Registry {
    //<ID, <ACTIVITY_RESULT, DATA>>
    private val resolvers: HashMap<String, CompletableFuture<*>> = HashMap()

    @RequiresApi(Build.VERSION_CODES.N)
    fun <T> new(): Pair<String, CompletionStage<Pair<Int, T>>> {
        val id = UUID.randomUUID().toString()
        val resultWithCode = CompletableFuture<Pair<Int, T>>()

        synchronized(resolvers) {
            if (resolvers.put(id, resultWithCode) != null) {
                throw RuntimeException("Why is the ID duplicate? Report the bug, please")
            }
        }

        return Pair(id, resultWithCode)
    }

    @RequiresApi(Build.VERSION_CODES.N)
    fun <T> resolve(id: String, reply: Pair<Int, T>) {
        @Suppress("UNCHECKED_CAST") //yes, it's how it should be
        val resolver = synchronized(resolvers) {
            resolvers.remove(id)
                ?: throw RuntimeException("No resolver for ID. Please, report the bug")
        } as? CompletableFuture<Pair<Int, T>>

        (resolver ?: throw RuntimeException("Don't hack the private API")).complete(reply)
    }
}