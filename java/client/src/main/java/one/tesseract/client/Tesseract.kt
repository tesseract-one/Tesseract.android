package one.tesseract.client

import one.tesseract.TesseractCommon

class Tesseract(private val ptr: Long = 0, delegate: Delegate): TesseractCommon() {
    init {
        create(delegate)
    }

    companion object {
        fun default(): Tesseract = Tesseract(delegate = DefaultDelegate())
    }

    private external fun create(delegate: Delegate)
    protected external fun finalize()
}