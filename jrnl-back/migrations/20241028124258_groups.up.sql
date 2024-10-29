CREATE TABLE IF NOT EXISTS groups
(
    id       UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name     VARCHAR(255) NOT NULL,
    code     TEXT         NOT NULL UNIQUE,
    owner_id UUID         NOT NULL REFERENCES profiles (id) ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS group_memberships
(
    group_id UUID NOT NULL REFERENCES groups (id) ON DELETE CASCADE,
    user_id  UUID NOT NULL REFERENCES profiles (id) ON DELETE CASCADE,
    PRIMARY KEY (group_id, user_id)
);

-- auto create a group membership for the owner when the group is initially created
CREATE OR REPLACE FUNCTION handle_new_group() RETURNS trigger
    LANGUAGE plpgsql AS
$$
BEGIN
    INSERT INTO group_memberships (group_id, user_id) VALUES (NEW.id, NEW.owner_id);
    RETURN NEW;
END;
$$;

CREATE TRIGGER on_group_created
    AFTER INSERT
    ON groups
    FOR EACH ROW
EXECUTE FUNCTION handle_new_group();

ALTER TABLE groups
    ENABLE ROW LEVEL SECURITY;

ALTER TABLE group_memberships
    ENABLE ROW LEVEL SECURITY;

CREATE INDEX idx_group_memberships_user_id ON group_memberships (user_id);
CREATE INDEX idx_groups_code ON groups (code);
