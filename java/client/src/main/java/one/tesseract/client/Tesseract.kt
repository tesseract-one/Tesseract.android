package one.tesseract.client

import one.tesseract.TesseractCommon

class Tesseract(private val ptr: Long = 0, delegate: Delegate): TesseractCommon() {
    init {
        create(delegate)
    }

    private external fun create(delegate: Delegate)
    protected external fun finalize()
}