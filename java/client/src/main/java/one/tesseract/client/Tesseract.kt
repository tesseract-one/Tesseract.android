package one.tesseract.client

import android.app.Application
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.GlobalScope
import one.tesseract.TesseractCommon
import one.tesseract.client.kotlin.DelegateAdapter
import kotlin.reflect.KClass

import one.tesseract.client.java.Delegate as JDelegate
import one.tesseract.client.kotlin.Delegate as KDelegate

import one.tesseract.service.java.Service as JService
import one.tesseract.service.kotlin.Service as KService

class Tesseract
    private constructor(
        @Suppress("unused") private val ptr: Long,
        private val scope: CoroutineScope,
        delegate: JDelegate?,
        val application: Application): TesseractCommon() {

    @OptIn(DelicateCoroutinesApi::class)
    constructor(scope: CoroutineScope = GlobalScope, delegate: JDelegate?, application: Application):
            this(ptr = 0, scope = scope, delegate = delegate, application)

    @OptIn(DelicateCoroutinesApi::class)
    constructor(scope: CoroutineScope = GlobalScope, delegate: KDelegate? = null, application: Application):
            this(scope = scope, delegate = delegate?.let { DelegateAdapter(it, scope) }, application)

    init {
        create(delegate)
    }

    companion object {
        fun default(application: Application): Tesseract = Tesseract(application = application)
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

    private external fun <T: JService> service(name: String): T

    private external fun create(delegate: JDelegate?)
    protected external fun finalize()
}