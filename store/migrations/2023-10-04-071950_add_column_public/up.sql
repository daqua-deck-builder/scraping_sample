-- Your SQL goes here
ALTER TABLE "card"
    ADD "public" boolean NOT NULL DEFAULT false;
COMMENT ON TABLE "card" IS '';