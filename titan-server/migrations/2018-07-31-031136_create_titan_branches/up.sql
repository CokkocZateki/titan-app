CREATE TABLE titan_branches (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(255) UNIQUE,
  wcf_user_group_id INTEGER UNIQUE,
  is_enabled BOOL DEFAULT true
);

-- The wcf_user_group_id values map to the wcf1_user_group table in unkso_primary database.
INSERT INTO titan_branches (name, wcf_user_group_id) VALUES ('Army', 28);
INSERT INTO titan_branches (name, wcf_user_group_id, is_enabled) VALUES ('Air Force', 32, false);
INSERT INTO titan_branches (name, wcf_user_group_id, is_enabled) VALUES ('Marines', 36, false);
INSERT INTO titan_branches (name, wcf_user_group_id) VALUES ('Navy', 40);
INSERT INTO titan_branches (name, wcf_user_group_id) VALUES ('Coast Guard', 69);