CREATE TYPE "public"."output_style" AS ENUM('Compact', 'Detailed', 'Minimal');--> statement-breakpoint
CREATE TABLE "user_preferences" (
	"user_id" varchar(18) PRIMARY KEY NOT NULL,
	"style" "output_style" DEFAULT 'Compact' NOT NULL
);
