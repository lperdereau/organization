-- Your SQL goes here
CREATE TABLE groups (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  organization_id uuid REFERENCES organizations(id) NOT NULL,
  name VARCHAR NOT NULL,
  CONSTRAINT pk_groups PRIMARY KEY ( id )
);
