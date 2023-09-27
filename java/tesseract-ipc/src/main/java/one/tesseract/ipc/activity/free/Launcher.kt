package one.tesseract.ipc.activity.free

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import kotlinx.coroutines.Deferred
import kotlinx.coroutines.future.asDeferred
import one.tesseract.ipc.activity.ActivityMonitor
import one.tesseract.ipc.activity.free.Launcher.Companion.LAUNCH_ID
import java.util.concurrent.CompletionStage

public class Launcher(private val monitor: ActivityMonitor) {
    companion object {
        const val LAUNCH_ID = "LAUNCH_ID"
    }

    public suspend fun <T> startFreeActivityForResult(
        activity: Class<out Activity>,
        bundle: Bundle?
    ): Pair<Int, T> {
        return startFreeActivityForResultDeferred<T>(activity, bundle).await()
    }

    public fun <T> startFreeActivityForResultDeferred(
        activity: Class<out Activity>,
        bundle: Bundle?
    ): Deferred<Pair<Int, T>> {
        return startFreeActivityForResultFuture<T>(activity, bundle).asDeferred()
    }

    //<RESULT_CODE, DATA>

    public fun <T> startFreeActivityForResultFuture(
        activity: Class<out Activity>,
        bundle: Bundle?
    ): CompletionStage<Pair<Int, T>> {
        val parent = monitor.activity
        val (id, future) = Registry.new<T>()

        val params = Bundle()
        bundle?.also { params.putAll(it) }
        params.putString(LAUNCH_ID, id)

        val intent = Intent(parent.applicationContext, activity)
        intent.putExtras(params)

        parent.startActivity(intent)

        return future
    }
}

fun <T> Activity.returnFreeResult(code: Int, result: T) {
    val id = intent.extras?.getString(LAUNCH_ID)
        ?: throw RuntimeException("FreeActivity wasn't launched using startFreeActivityForResult method")
    Registry.resolve(id, Pair(code, result))
}

fun <T> Activity.finishFreeActivity(code: Int, result: T) {
    returnFreeResult(code, result)
    finish()
}