CREATE TABLE "entries" (
"id" uuid NOT NULL PRIMARY KEY,
"tags" text[],
"text" text NOT NULL,
"created_at" timestamptz NOT NULL DEFAULT NOW(),
"updated_at" timestamptz NOT NULL DEFAULT NOW()
)