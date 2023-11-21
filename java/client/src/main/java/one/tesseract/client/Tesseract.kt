package one.tesseract.client

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.GlobalScope
import one.tesseract.TesseractCommon
import one.tesseract.client.kotlin.DelegateAdapter
import one.tesseract.client.java.Delegate as JDelegate
import one.tesseract.client.kotlin.Delegate as KDelegate

class Tesseract
    @OptIn(DelicateCoroutinesApi::class) private constructor(
        @Suppress("unused") private val ptr: Long,
        private val scope: CoroutineScope = GlobalScope,
        delegate: JDelegate): TesseractCommon() {

    @OptIn(DelicateCoroutinesApi::class)
    constructor(scope: CoroutineScope = GlobalScope, delegate: JDelegate):
            this(ptr = 0, scope = scope, delegate = delegate)

    @OptIn(DelicateCoroutinesApi::class)
    constructor(scope: CoroutineScope = GlobalScope, delegate: KDelegate):
            this(scope = scope, delegate = DelegateAdapter(delegate, scope))

    init {
        create(delegate)
    }

    companion object {
        fun default(): Tesseract = Tesseract(delegate = DefaultDelegate())
    }

    private external fun create(delegate: JDelegate)
    protected external fun finalize()
}