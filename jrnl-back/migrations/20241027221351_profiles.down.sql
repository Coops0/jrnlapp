DROP TABLE IF EXISTS public.profiles;
DROP FUNCTION IF EXISTS handle_new_user();
DROP TRIGGER IF EXISTS on_auth_user_created ON auth.users;