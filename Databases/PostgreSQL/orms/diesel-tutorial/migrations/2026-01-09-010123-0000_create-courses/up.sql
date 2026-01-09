CREATE TABLE courses
(
    id    SERIAL PRIMARY KEY,
    title VARCHAR UNIQUE
);

CREATE TABLE enrollments
(
    student_id INTEGER REFERENCES users (id) ON DELETE CASCADE,
    course_id  INTEGER REFERENCES courses (id) ON DELETE CASCADE,
    PRIMARY KEY (student_id, course_id)
);

--- Create courses, users, and enrollments with upsert helpers

CREATE FUNCTION upsert_course(p_title VARCHAR) RETURNS INTEGER AS
$$
INSERT INTO courses (title)
VALUES (p_title)
ON CONFLICT (title) DO UPDATE
    SET title = EXCLUDED.title
RETURNING id;
$$ LANGUAGE sql; -- simple single-statement function


-- Upsert helper for users (assumes a unique constraint on users.email)
CREATE FUNCTION upsert_user(p_email VARCHAR, p_name VARCHAR) RETURNS INTEGER AS
$$
INSERT INTO users (email, name)
VALUES (p_email, p_name)
ON CONFLICT (email) DO UPDATE
    SET name = EXCLUDED.name
RETURNING id;
$$ LANGUAGE sql;

CREATE FUNCTION enroll_student_in_course(p_email VARCHAR, p_name VARCHAR, p_course_title VARCHAR) RETURNS VOID AS
$$
DECLARE
    v_student_id INTEGER;
    v_course_id  INTEGER;
BEGIN
    v_student_id := upsert_user(p_email, p_name);
    v_course_id := upsert_course(p_course_title);

    INSERT INTO enrollments (student_id, course_id)
    VALUES (v_student_id, v_course_id)
    ON CONFLICT (student_id, course_id) DO NOTHING;
END;
$$ LANGUAGE plpgsql; -- multiple statements, assignments

DO
$$
    DECLARE
        rec RECORD;
    BEGIN
        FOR rec IN
            SELECT *
            FROM (VALUES ('alice@example.com', 'Alice', 'Rust 101'),
                         ('bob@example.com', 'Bob', 'SQL Basics'),
                         ('carol@example.com', 'Carol', 'Advanced Rust'),
                         ('dave@example.com', 'Dave', 'Data Structures'),
                         ('erin@example.com', 'Erin', 'Databases'),
                         ('frank@example.com', 'Frank', 'Web Development'),
                         ('grace@example.com', 'Grace', 'Algorithms'),
                         ('heidi@example.com', 'Heidi', 'Systems Programming'),
                         ('ivan@example.com', 'Ivan', 'Concurrency'),
                         ('judy@example.com', 'Judy', 'Testing'),
                         ('karl@example.com', 'Karl', 'Functional Programming'),
                         ('leah@example.com', 'Leah', 'DevOps'),
                         ('mateo@example.com', 'Mateo', 'Machine Learning'),
                         ('nina@example.com', 'Nina', 'Cloud Fundamentals'),
                         ('omar@example.com', 'Omar', 'Security'),
                         ('priya@example.com', 'Priya', 'Networking'),
                         ('quentin@example.com', 'Quentin', 'CLI Tools'),
                         ('rosa@example.com', 'Rosa', 'Embedded Rust'),
                         ('sam@example.com', 'Sam', 'Async Rust'),
                         ('tilda@example.com', 'Tilda', 'Performance Tuning'))
                     AS v(email, name, course_title)
            LOOP
                PERFORM enroll_student_in_course(rec.email, rec.name, rec.course_title);
            END LOOP;
    END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION enroll_student(p_email VARCHAR, p_course_title VARCHAR) RETURNS VOID AS
$$
DECLARE
    v_student_id INTEGER;
    v_course_id  INTEGER;
BEGIN
    v_student_id := (SELECT id FROM users WHERE email = p_email);
    IF v_student_id IS NULL THEN
        RAISE EXCEPTION 'Student with email % does not exist', p_email;
    END IF;
    v_course_id := (SELECT id FROM courses WHERE title = p_course_title);
    IF v_course_id IS NULL THEN
        RAISE EXCEPTION 'Course with title % does not exist', p_course_title;
    END IF;
    INSERT INTO enrollments (student_id, course_id)
    VALUES (v_student_id, v_course_id)
    ON CONFLICT (student_id, course_id) DO NOTHING;
END;
$$ LANGUAGE plpgsql;

DO
$$
    DECLARE
        rec RECORD;
    BEGIN
        FOR rec IN
            SELECT *
            FROM (VALUES ('alice@example.com', 'Rust 101'),
                         ('alice@example.com', 'Advanced Rust'),
                         ('alice@example.com', 'Async Rust'),
                         ('bob@example.com', 'Databases'),
                         ('bob@example.com', 'SQL Basics'),
                         ('carol@example.com', 'Advanced Rust'),
                         ('carol@example.com', 'Rust 101'),
                         ('dave@example.com', 'Data Structures'),
                         ('dave@example.com', 'Algorithms'),
                         ('erin@example.com', 'Databases'),
                         ('erin@example.com', 'SQL Basics'),
                         ('frank@example.com', 'Web Development'),
                         ('frank@example.com', 'CLI Tools'),
                         ('grace@example.com', 'Algorithms'),
                         ('grace@example.com', 'Data Structures'),
                         ('heidi@example.com', 'Systems Programming'),
                         ('heidi@example.com', 'Embedded Rust'),
                         ('ivan@example.com', 'Concurrency'),
                         ('ivan@example.com', 'Async Rust'),
                         ('judy@example.com', 'Testing'),
                         ('karl@example.com', 'Functional Programming'),
                         ('leah@example.com', 'DevOps'),
                         ('mateo@example.com', 'Machine Learning'),
                         ('nina@example.com', 'Cloud Fundamentals'),
                         ('omar@example.com', 'Security'),
                         ('priya@example.com', 'Networking'),
                         ('quentin@example.com', 'CLI Tools'),
                         ('rosa@example.com', 'Embedded Rust'),
                         ('sam@example.com', 'Async Rust'),
                         ('tilda@example.com', 'Performance Tuning')) AS v(email, course_title)
            LOOP
                PERFORM enroll_student(rec.email, rec.course_title);
            END LOOP;
    END;
$$ LANGUAGE plpgsql;