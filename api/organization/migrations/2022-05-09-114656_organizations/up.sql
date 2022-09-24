-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE organizations (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  name VARCHAR NOT NULL,
  CONSTRAINT pk_organizations PRIMARY KEY ( id )
);
