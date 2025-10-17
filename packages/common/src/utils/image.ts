import sharp from "sharp";

export async function getImageDominantColor(
	imageUrl: string,
): Promise<[number, number, number]> {
	const res = await Bun.fetch(imageUrl);
	const buf = Buffer.from(await res.arrayBuffer());

	const { data } = await sharp(buf, { failOnError: false })
		.resize(150, 150, { fit: "inside", kernel: "nearest" })
		.ensureAlpha()
		.raw()
		.toBuffer({ resolveWithObject: true });

	return getDominantColor(data);
}

/**
 * Compute the dominant color from an RGBA pixel buffer using a fixed-size histogram.
 *
 * Each color channel (R, G, B) is quantized to 5 bits (32 possible values),
 * resulting in 32 × 32 × 32 = 32,768 possible color bins.
 *
 * This method performs a single linear pass through all pixels,
 * counting occurrences in the histogram, and then finds the most frequent bin.
 *
 * @param rgba - RGBA pixel data as a Uint8Array.
 * @returns The dominant color as an RGB array (e.g. [123, 45, 67]).
 */
export function getDominantColor(rgba: Uint8Array): [number, number, number] {
	// 32^3 = 32768 possible quantized RGB combinations
	const histogram = new Uint32Array(32768);

	for (let i = 0; i < rgba.length; i += 4) {
		const r = rgba[i] ?? 0;
		const g = rgba[i + 1] ?? 0;
		const b = rgba[i + 2] ?? 0;

		// Quantize each channel to 5 bits
		const rq = r >> 3;
		const gq = g >> 3;
		const bq = b >> 3;

		// Build unique key in range [0, 32767]
		const key = (rq << 10) | (gq << 5) | bq;

		// Increment histogram bin
		histogram[key] = (histogram[key] ?? 0) + 1;
	}

	let maxKey = 0;
	let maxCount = 0;

	// Find the bin with the highest frequency
	for (let i = 0; i < 32768; i++) {
		const count = histogram[i];
		if (count !== undefined && count > maxCount) {
			maxCount = count;
			maxKey = i;
		}
	}

	// Decode the quantized key back into approximate RGB
	const r = ((maxKey >> 10) & 31) << 3;
	const g = ((maxKey >> 5) & 31) << 3;
	const b = (maxKey & 31) << 3;

	return [r, g, b];
}
