import { env } from "./env";

export const Database = {
	host: env.DB_HOST,
	username: env.DB_USER,
	password: env.DB_PASSWORD,
	database: env.DB_NAME,
	port: env.DB_PORT,

	toUrl(): string {
		return `postgres://${this.username}:${this.password}@${this.host}:${this.port}/${this.database}`;
	},

	toUrlSafe(): string {
		const labels = this.host.split(".");
		let maskedHost: string;
		if (labels.length > 2) {
			const masked = Array(labels.length - 2).fill("***");
			masked.push(...labels.slice(-2));
			maskedHost = masked.join(".");
		} else if (labels.length === 2) {
			maskedHost = `***.${labels[1]}`;
		} else {
			maskedHost = "***";
		}
		return `postgres://${this.username}@${maskedHost}:${this.port}/${this.database}`;
	},
};

export const Prefixes = {
	default: env.PREFIX_DEFAULT,
	development: env.PREFIX_DEV,

	get(): string {
		return env.NODE_ENV === "production" ? this.default : this.development;
	},
};
