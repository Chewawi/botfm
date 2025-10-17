import { env } from "@repo/common/config";

export function buildQueryString(params: [string, string][]): string {
	return params
		.map(
			([key, value]) =>
				`${encodeURIComponent(key)}=${encodeURIComponent(value)}`,
		)
		.join("&");
}

export function generateSignature(params: [string, string][]): string {
	const sorted = [...params].sort((a, b) => a[0].localeCompare(b[0]));
	const raw = sorted.map(([k, v]) => k + v).join("") + env.LASTFM_SECRET;
	return new Bun.CryptoHasher("md5").update(raw, "utf8").digest("hex");
}

export async function fetchJson<T>(url: string): Promise<T> {
	const response = await Bun.fetch(url, {
		method: "GET",
		headers: { Accept: "application/json" },
	});

	if (!response.ok) {
		const text = await response.text();
		throw new Error(`HTTP ${response.status}: ${text}`);
	}

	return response.json() as Promise<T>;
}
