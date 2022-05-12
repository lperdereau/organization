-- Your SQL goes here
CREATE TABLE groups (
  id uuid NOT NULL DEFAULT uuid_generate_v4(),
  organization_id uuid  NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  name VARCHAR NOT NULL,
  CONSTRAINT pk_groups PRIMARY KEY ( id )
);

-- Your SQL goes here
CREATE TABLE users_groups (
  user_id uuid REFERENCES users(id) ON DELETE CASCADE,
  group_id uuid REFERENCES groups(id) ON DELETE CASCADE,
  PRIMARY KEY (user_id, group_id)
);
