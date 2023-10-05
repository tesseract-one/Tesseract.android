package one.tesseract.service

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.GlobalScope

import one.tesseract.ipc.service.IPCTransport
import one.tesseract.transport.service.Transport

import one.tesseract.service.service.java.Service as JavaService
import one.tesseract.service.service.kotlin.Service as KotlinService

open class Tesseract @OptIn(DelicateCoroutinesApi::class) constructor(@Suppress("unused") private val ptr: Long = 0, private val scope: CoroutineScope = GlobalScope) {
    companion object {
        init {
            System.loadLibrary("tesseract")
        }

        @OptIn(DelicateCoroutinesApi::class)
        fun default(scope: CoroutineScope = GlobalScope): Tesseract =
            Tesseract(scope = scope).transport(IPCTransport())
    }

    init {
        create()
    }

    private external fun create()

    @Synchronized
    external fun service(service: JavaService): Tesseract

    fun service(service: KotlinService): Tesseract = service(service.toJava(scope))

    @Synchronized
    external fun transport(transport: Transport): Tesseract

    protected external fun finalize()
}