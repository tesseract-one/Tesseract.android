package one.tesseract.client

import android.app.Application
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.GlobalScope
import kotlin.reflect.KClass

import one.tesseract.TesseractCommon
import one.tesseract.client.kotlin.DelegateAdapter
import one.tesseract.client.transport.ClientTransport
import one.tesseract.client.transport.JavaTransport
import one.tesseract.client.transport.ipc.IPCTransport

import one.tesseract.client.java.Delegate as JDelegate
import one.tesseract.client.kotlin.Delegate as KDelegate

import one.tesseract.service.java.Service as JService
import one.tesseract.service.kotlin.Service as KService

class Tesseract
    private constructor(
        @Suppress("unused") private val ptr: Long,
        private val scope: CoroutineScope,
        delegate: JDelegate?): TesseractCommon() {

    @OptIn(DelicateCoroutinesApi::class)
    constructor(scope: CoroutineScope = GlobalScope, delegate: JDelegate?):
            this(ptr = 0, scope = scope, delegate = delegate)

    @OptIn(DelicateCoroutinesApi::class)
    constructor(scope: CoroutineScope = GlobalScope, delegate: KDelegate? = null):
            this(scope = scope, delegate = delegate?.let { DelegateAdapter(it, scope) })

    init {
        create(delegate)
    }

    companion object {
        fun default(application: Application, delegate: KDelegate? = null): Tesseract =
            Tesseract(delegate = delegate).transport(IPCTransport(application))
        fun default(application: Application, delegate: JDelegate): Tesseract =
            Tesseract(delegate = delegate).transport(IPCTransport(application))
    }

    fun <T: JService> service(service: Class<T>): T {
        if(!service.isInterface) {
            throw RuntimeException("Put one of the designated interfaces from: one.tesseract.protocol.java")
        }
        return service(service.simpleName)
    }

    fun <T: KService> service(service: KClass<T>): T {
        if(!service.java.isInterface) {
            throw RuntimeException("Put one of the designated interfaces from: one.tesseract.protocol.kotlin")
        }
        val instance: JService = service(service.java.simpleName)
        @Suppress("UNCHECKED_CAST")
        return instance.toKotlin() as T
    }

    @Synchronized
    external fun transport(transport: ClientTransport): Tesseract

    private external fun <T: JService> service(name: String): T

    private external fun create(delegate: JDelegate?)
    protected external fun finalize()
}