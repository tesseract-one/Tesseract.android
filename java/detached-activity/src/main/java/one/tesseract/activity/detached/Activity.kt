package one.tesseract.activity.detached

import android.app.Activity
import android.os.Bundle

fun Activity.getExtras(): Bundle? = intent.extras

fun <T> Activity.returnResultDetached(code: Int, result: T) {
    val id = getExtras()?.getString(Launcher.LAUNCH_ID)
        ?: throw RuntimeException("FreeActivity wasn't launched using startFreeActivityForResult method")
    Registry.resolve(id, Pair(code, result))
}

fun <T> Activity.finishDetachedActivity(code: Int, result: T) {
    returnResultDetached(code, result)
    finish()
}