package one.tesseract.activity.detached

import java.util.concurrent.CompletionStage

import kotlinx.coroutines.Deferred
import kotlinx.coroutines.future.asDeferred

import android.app.Activity
import android.app.Application
import android.content.Intent
import android.os.Bundle

import one.tesseract.activity.ActivityMonitor

class Launcher(private val monitor: ActivityMonitor) {
    companion object {
        const val LAUNCH_ID = "LAUNCH_ID"
    }

    constructor(application: Application) : this(ActivityMonitor(application))

    suspend fun <T> startDetachedActivityForResult(
        activity: Class<out Activity>,
        extras: Bundle?
    ): Pair<Int, T> {
        return startDetachedActivityForResultDeferred<T>(activity, extras).await()
    }

    fun <T> startDetachedActivityForResultDeferred(
        activity: Class<out Activity>,
        extras: Bundle?
    ): Deferred<Pair<Int, T>> {
        return startDetachedActivityForResultFuture<T>(activity, extras).asDeferred()
    }

    //<RESULT_CODE, DATA>

    fun <T> startDetachedActivityForResultFuture(
        activity: Class<out Activity>,
        extras: Bundle?
    ): CompletionStage<Pair<Int, T>> {
        val parent = monitor.activity
        val (id, future) = Registry.new<T>()

        val params = Bundle()
        extras?.also { params.putAll(it) }
        params.putString(LAUNCH_ID, id)

        val intent = Intent(parent.applicationContext, activity)
        intent.putExtras(params)

        parent.startActivity(intent)

        return future
    }
}
