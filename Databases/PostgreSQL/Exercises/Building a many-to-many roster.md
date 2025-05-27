## Defining tables
```postgresql
DROP TABLE IF EXISTS student;
CREATE TABLE student (
    id SERIAL,
    name VARCHAR(128) UNIQUE,
    PRIMARY KEY(id)
);

DROP TABLE IF EXISTS course CASCADE;
CREATE TABLE course (
    id SERIAL,
    title VARCHAR(128) UNIQUE,
    PRIMARY KEY(id)
);

DROP TABLE IF EXISTS roster CASCADE;
CREATE TABLE roster (
    id SERIAL,
    student_id INTEGER REFERENCES student(id) ON DELETE CASCADE,
    course_id INTEGER REFERENCES course(id) ON DELETE CASCADE,
    role INTEGER,
    UNIQUE(student_id, course_id),
    PRIMARY KEY (id)
);
```

## Generating commands

```python
text = """Ariel, si106, Instructor  
Brooklin, si106, Learner  
Cain, si106, Learner  
Kimberly, si106, Learner  
Zakariya, si106, Learner  
Spondon, si110, Instructor  
Baron, si110, Learner  
Barry, si110, Learner  
Billi, si110, Learner  
Jeya, si110, Learner  
Zuzanna, si206, Instructor  
Ayiah, si206, Learner  
Nawal, si206, Learner  
Prithvi, si206, Learner  
Sereen, si206, Learner"""  
  
lines = text.strip().split('\n')  
data = []  
courses = set()  
names = set()  
  
  
def insert_values_generator(table_name, column_name, values):  
    values_str = ", ".join(f"('{value}')" for value in values)  
    return f"INSERT INTO {table_name} ({column_name}) VALUES {values_str};"  
  
  
for line in lines:  
    name, course, role = line.split(', ')  
    data.append({'name': name, 'course': course, 'role': role})  
    courses.add(course)  
    names.add(name)  
## Populate course table  
print(insert_values_generator('course', 'title', sorted(courses)))  
## Populate person table  
print(insert_values_generator('student', 'name', sorted(names)))  
courses = {course: i + 1 for i, course in enumerate(sorted(courses))}  
students = {name: i + 1 for i, name in enumerate(sorted(names))}  
roles = {'Instructor': 1, 'Learner': 0}  
## Populate course_person table  
data = ", ".join(str((students[row["name"]], courses[row["course"]], roles[row["role"]])) for row in data)  
print(f"INSERT INTO roster (student_id,course_id,role) VALUES {data};")
```

## Executing commands
```postgresql
INSERT INTO course (title) VALUES ('si106'), ('si110'), ('si206');
INSERT INTO student (name) VALUES ('Ariel'), ('Ayiah'), ('Baron'), ('Barry'), ('Billi'), ('Brooklin'), ('Cain'), ('Jeya'), ('Kimberly'), ('Nawal'), ('Prithvi'), ('Sereen'), ('Spondon'), ('Zakariya'), ('Zuzanna');
INSERT INTO roster (student_id, course_id, role) VALUES (1, 1, 1), (6, 1, 0), (7, 1, 0), (9, 1, 0), (14, 1, 0), (13, 2, 1), (3, 2, 0), (4, 2, 0), (5, 2, 0), (8, 2, 0), (15, 3, 1), (2, 3, 0), (10, 3, 0), (11, 3, 0), (12, 3, 0);
```

## Retrieving data

```postgresql
SELECT student.name, course.title, roster.role
    FROM student 
    JOIN roster ON student.id = roster.student_id
    JOIN course ON roster.course_id = course.id
    ORDER BY course.title, roster.role DESC, student.name;
```
