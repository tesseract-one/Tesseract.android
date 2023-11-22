package one.tesseract.service.java

import one.tesseract.service.kotlin.Service as KotlinService

interface Service {
    fun toKotlin(): KotlinService
}