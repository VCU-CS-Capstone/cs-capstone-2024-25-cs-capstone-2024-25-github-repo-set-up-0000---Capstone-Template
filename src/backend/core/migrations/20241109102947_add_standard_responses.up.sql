-- Add up migration script here
CREATE TABLE IF NOT EXISTS _default_questions(
    id serial PRIMARY KEY,
    file_name VARCHAR(255) NOT NULL,
    added_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS locations(
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    program VARCHAR(255) NOT NULL,
    parent_location INTEGER,
        CONSTRAINT FK_locations_parent_location
            FOREIGN KEY (parent_location)
            REFERENCES locations(id)
            ON DELETE CASCADE
);

INSERT INTO locations(name, program) VALUES
    ('Church Hill House', 'RHWP'),
    ('Dominion Place', 'RHWP'),
    ('Highland Park', 'RHWP'),
    ('4th Ave', 'RHWP'),
    ('Health Hub', 'RHWP'),
    ('The Rosa', 'RHWP'),
    ('Petersburg', 'MHWP'),
    ('Lawrenceville','MHWP'),
    ('Tappahannock', 'MHWP'),
    ('Southwood', 'MHWP');

-- Take the primary key from petersburg and insert setting the parent location
-- VCRC
-- Police substation
-- Gilhaven
-- VSU Van
INSERT INTO locations(name, program, parent_location) VALUES
    ('VCRC', 'MHWP', (SELECT id FROM locations WHERE name = 'Petersburg' LIMIT 1)),
    ('Police substation', 'MHWP', (SELECT id FROM locations WHERE name = 'Petersburg' LIMIT 1)),
    ('Gilhaven', 'MHWP', (SELECT id FROM locations WHERE name = 'Petersburg' LIMIT 1)),
    ('VSU Van', 'MHWP', (SELECT id FROM locations WHERE name = 'Petersburg' LIMIT 1));


CREATE TABLE question_categories(
    id serial PRIMARY KEY,
    string_id VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT
);
CREATE TABLE questions(
    id serial PRIMARY KEY,
    category_id INTEGER,
        CONSTRAINT FK_questions_category
            FOREIGN KEY (category_id)
            REFERENCES question_categories(id)
            ON DELETE SET NULL,
    question_type VARCHAR(255) NOT NULL,
    question VARCHAR(255) NOT NULL,
    description TEXT,
    red_cap_id VARCHAR(255) UNIQUE,
    red_cap_other_id VARCHAR(255) UNIQUE,
    removed BOOLEAN DEFAULT FALSE
);


CREATE TABLE IF NOT EXISTS question_options(
    id serial PRIMARY KEY,
    question_id INTEGER NOT NULL,
        CONSTRAINT FK_question_options_question_id
            FOREIGN KEY (question_id)
            REFERENCES questions(id)
            ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    red_cap_option_index INTEGER,
    removed BOOLEAN DEFAULT FALSE
);
CREATE TABLE IF NOT EXISTS question_requirements(
    id serial PRIMARY KEY,
    question_to_check INTEGER NOT NULL,
        CONSTRAINT FK_question_requirements_question_to_check
            FOREIGN KEY (question_to_check)
            REFERENCES questions(id)
            ON DELETE CASCADE,
    question_to_add INTEGER NOT NULL,
        CONSTRAINT FK_question_requirements_question_to_add
            FOREIGN KEY (question_to_add)
            REFERENCES questions(id)
            ON DELETE CASCADE,
    has_option INTEGER,
        CONSTRAINT FK_question_requirements_has_option
            FOREIGN KEY (has_option)
            REFERENCES question_options(id)
            ON DELETE CASCADE,
    equals_radio INTEGER,
        CONSTRAINT FK_question_requirements_equals_radio
            FOREIGN KEY (equals_radio)
            REFERENCES question_options(id)
            ON DELETE CASCADE,
    equals_boolean BOOLEAN,
    equals_text TEXT,
    equals_number INTEGER
);




