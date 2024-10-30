CREATE TABLE IF NOT EXISTS profiles
(
    id       UUID                   NOT NULL REFERENCES auth.users ON DELETE CASCADE,
    name     TEXT                   NOT NULL,
    theme    TEXT DEFAULT 'default' NOT NULL,
    timezone TEXT DEFAULT 'UTC'     NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE profiles
    ENABLE ROW LEVEL SECURITY;

CREATE OR REPLACE FUNCTION handle_new_user()
    RETURNS trigger
    LANGUAGE plpgsql
    SECURITY DEFINER SET search_path = public
AS
$$
BEGIN
    INSERT INTO profiles (id, name)
    VALUES (NEW.id, NEW.raw_user_meta_data ->> 'name');
    RETURN NEW;
END;
$$;

CREATE OR REPLACE TRIGGER on_auth_user_created
    AFTER INSERT
    ON auth.users
    FOR EACH ROW
EXECUTE FUNCTION handle_new_user();