DROP TRIGGER IF EXISTS set_users_updated ON users;
DROP FUNCTION IF EXISTS update_users_updated_column();

DROP TABLE IF EXISTS users;
DROP TYPE IF EXISTS user_role;
