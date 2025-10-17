export * from "./image";

/**
 * Truncates a string to a maximum length.
 * @param text The text to truncate
 * @param maxLength Maximum allowed length
 * @returns The truncated string, with "..." appended if it was too long
 */
export function truncateText(text: string, maxLength: number): string {
	if (text.length > maxLength) {
		return `${text.slice(0, maxLength - 3)}...`;
	} else {
		return text;
	}
}
