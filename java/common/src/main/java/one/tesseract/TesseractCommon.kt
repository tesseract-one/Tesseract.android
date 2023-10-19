package one.tesseract

import android.annotation.SuppressLint

open class TesseractCommon {
    @SuppressLint("UseValueOf")
    companion object {
        @SuppressLint("UseValueOf")
        @Suppress("PLATFORM_CLASS_MAPPED_TO_KOTLIN", "DEPRECATION")
        private var initialized: java.lang.Boolean = java.lang.Boolean(false)

        init {
            synchronized(initialized) {
                if(!initialized.booleanValue()) {
                    System.loadLibrary("tesseract")
                    runtimeInit()
                    @Suppress("DEPRECATION")
                    initialized = java.lang.Boolean(true)
                }
            }
        }

        @JvmStatic
        private external fun runtimeInit()
    }
}