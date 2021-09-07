package io.parity.signer.models

import android.graphics.BitmapFactory
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap

/**
 * Decodes from hex string into number array
 */
fun String.decodeHex(): ByteArray {
	return chunked(2).map { it.toInt(16).toByte() }.toByteArray()
}

/**
 * Specialized tool to decode png images generated by rust code
 */
fun String.intoImageBitmap(): ImageBitmap {
	val picture = this.decodeHex()
	return BitmapFactory.decodeByteArray(picture, 0, picture.size).asImageBitmap()
}
