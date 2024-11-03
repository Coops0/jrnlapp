DROP TABLE IF EXISTS groups CASCADE;
DROP TABLE IF EXISTS group_memberships CASCADE;

DROP INDEX IF EXISTS idx_group_memberships_user_id;
DROP INDEX IF EXISTS idx_groups_code;

DROP TRIGGER IF EXISTS on_group_created ON groups CASCADE;
DROP FUNCTION IF EXISTS handle_new_group CASCADE;