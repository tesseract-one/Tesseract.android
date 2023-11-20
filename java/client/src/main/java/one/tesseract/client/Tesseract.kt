package one.tesseract.client

import one.tesseract.TesseractCommon

class Tesseract(private val ptr: Long = 0): TesseractCommon() {
    private external fun create()
    protected external fun finalize()
}