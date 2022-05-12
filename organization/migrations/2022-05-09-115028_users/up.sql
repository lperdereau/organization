-- Your SQL goes here
CREATE TABLE users (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  name VARCHAR NOT NULL,
  CONSTRAINT pk_users PRIMARY KEY ( id )
);

CREATE TABLE users_organizations (
  user_id uuid REFERENCES users(id) ON DELETE CASCADE,
  organization_id uuid REFERENCES organizations(id) ON DELETE CASCADE,
  PRIMARY KEY (user_id, organization_id)
);
