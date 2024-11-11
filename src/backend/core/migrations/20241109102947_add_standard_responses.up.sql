-- Add up migration script here
CREATE TABLE IF NOT EXISTS locations(
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    program VARCHAR(255) NOT NULL
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
    ('The Rosa', 'RHWP')
    ('Petersburg', 'MHWP')
    ('Lawrenceville','MHWP'),
    ('Tappahannock', 'MHWP'),
    ('Southwood', 'MHWP');

-- Take the primary key from petersburg and insert setting the parent location
-- VCRC
-- Police substation
-- Gilhaven
-- VSU Van
INSERT INTO locations(name, program, parent_location) VALUES
    ('VCRC', 'MHWP', SELECT id FROM locations WHERE name = 'Petersburg'),
    ('Police substation', 'MHWP', SELECT id FROM locations WHERE name = 'Petersburg'),
    ('Gilhaven', 'MHWP', SELECT id FROM locations WHERE name = 'Petersburg'),
    ('VSU Van', 'MHWP', SELECT id FROM locations WHERE name = 'Petersburg');



CREATE TABLE IF NOT EXISTS disease_and_medication_education(
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT
);
-- Diabetes
-- Hypertension
-- Heart failure
-- Mental health
-- Medications
-- Mobility (wheelchair/walker safety)
-- Pain
-- Memory
-- Other
-- N/A - no disease/medication related education
INSERT INTO disease_and_medication_education(name) VALUES
    ('Diabetes'),
    ('Hypertension'),
    ('Heart failure'),
    ('Mental health'),
    ('Medications'),
    ('Mobility (wheelchair/walker safety)'),
    ('Pain'),
    ('Memory');

CREATE TABLE IF NOT EXISTS health_behavior_education(
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT
);

-- Diabetes management
-- Hypertension management
-- Managing heart failure
-- Medication adherence
-- Weight
-- Diet
-- Smoking reduction
-- Alcohol use
-- Pain Management
-- Use of blood pressure cuff
-- Goal setting
-- Mental Health

INSERT INTO health_behavior_education(name) VALUES
    ('Diabetes management'),
    ('Hypertension management'),
    ('Managing heart failure'),
    ('Medication adherence'),
    ('Weight'),
    ('Diet'),
    ('Smoking reduction'),
    ('Alcohol use'),
    ('Pain Management'),
    ('Use of blood pressure cuff'),
    ('Goal setting'),
    ('Mental Health');

