-- Function: get_users_enrolled_in_course(p_course_name TEXT)
-- Purpose: Return all users (id, name, email) enrolled in the course with the given title.
-- Params:
--   p_course_name TEXT - course title to match.
-- Returns:
--   TABLE(user_id INTEGER, user_name TEXT, user_email TEXT)
-- Notes:
--   Performs inner joins across users, enrollments, and courses.
--   Title match is exact; consider using ILIKE for case-insensitive matching if needed.
CREATE OR REPLACE FUNCTION get_users_enrolled_in_course(p_course_name TEXT)
    RETURNS TABLE
            (
                user_id    INTEGER,
                user_name  TEXT,
                user_email TEXT
            )
AS
$$
BEGIN
    RETURN QUERY
        SELECT u.id, u.name, u.email
        FROM users u
                 JOIN enrollments e ON e.student_id = u.id
                 JOIN courses c ON c.id = e.course_id
        WHERE c.title ILIKE '%' || p_course_name || '%';
END;
$$ LANGUAGE plpgsql STABLE;