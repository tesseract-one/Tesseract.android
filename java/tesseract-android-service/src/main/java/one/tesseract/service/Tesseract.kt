package one.tesseract.service

class Tesseract {
    companion object {
        init {
            System.loadLibrary("tesseract")
        }
    }
}