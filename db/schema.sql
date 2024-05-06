-- Use extension for UUID support.
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "user" (
  "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  "created_timestamp_unix" bigint NOT NULL,
  "email" text UNIQUE NOT NULL
);

CREATE TABLE "otp" (
  "id" serial PRIMARY KEY,
  "value" integer NOT NULL,
  "user_email" text NOT NULL,
  "exp_timestamp_unix" integer NOT NULL,
  "used_timestamp_unix" integer
);
