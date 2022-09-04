package one.tesseract.ipc.activity.free

import android.app.Activity
import android.content.Intent
import android.os.Build
import android.os.Bundle
import androidx.annotation.RequiresApi
import one.tesseract.ipc.activity.ActivityMonitor
import one.tesseract.ipc.activity.free.Launcher.Companion.LAUNCH_ID
import java.util.concurrent.CompletionStage

public class Launcher(private val monitor: ActivityMonitor) {
    companion object {
        const val LAUNCH_ID = "LAUNCH_ID"
    }

    //<RESULT_CODE, DATA>
    @RequiresApi(Build.VERSION_CODES.N)
    public fun <T> startFreeActivityForResult(
        activity: Class<out Activity>,
        bundle: Bundle?
    ): CompletionStage<Pair<Int, T>> {
        val parent = monitor.activity
        val (id, future) = Registry.new<T>()

        val params = Bundle()
        bundle?.also { params.putAll(it) }
        params.putString(LAUNCH_ID, id)

        var intent = Intent(parent.applicationContext, activity)
        intent.putExtras(params)

        parent.startActivity(intent)

        return future
    }
}

@RequiresApi(Build.VERSION_CODES.N)
fun <T> Activity.returnFreeResult(code: Int, result: T) {
    val id = intent.extras?.getString(LAUNCH_ID)
        ?: throw RuntimeException("FreeActivity wasn't launched using startFreeActivityForResult method")
    Registry.resolve(id, Pair(code, result))
}

@RequiresApi(Build.VERSION_CODES.N)
fun <T> Activity.finishFreeActivity(code: Int, result: T) {
    returnFreeResult(code, result)
    finish()
}