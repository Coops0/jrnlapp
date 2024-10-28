DROP TABLE IF EXISTS groups cascade;
DROP TABLE IF EXISTS group_memberships cascade;
DROP INDEX IF EXISTS idx_group_memberships_user_id;
DROP INDEX IF EXISTS idx_groups_code;
DROP TRIGGER IF EXISTS on_group_created ON groups cascade;
DROP FUNCTION IF EXISTS handle_new_group cascade;