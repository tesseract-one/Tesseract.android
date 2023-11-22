package one.tesseract.service.kotlin

import kotlinx.coroutines.CoroutineScope
import one.tesseract.service.java.Service as JavaService

interface Service {
    fun toJava(scope: CoroutineScope): JavaService
}