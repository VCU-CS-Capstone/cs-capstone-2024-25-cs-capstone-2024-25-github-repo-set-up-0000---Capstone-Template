-- Participants Table.
CREATE TABLE IF NOT EXISTS participants(
    -- serial 64 auto incrementing
    id serial PRIMARY KEY,
    red_cap_id INTEGER,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    -- Contact Info
    phone_number_one VARCHAR(255) ,
    phone_number_two VARCHAR(255) ,
    other_contact TEXT,
    -- Other Info
    program VARCHAR(255) NOT NULL,
    -- Relates to location table
    location INTEGER,
        CONSTRAINT FK_participants_location
            FOREIGN KEY (location)
            REFERENCES locations(id)
            ON DELETE SET NULL,
    status VARCHAR(255),
    behavioral_risks_identified TEXT,
    date_care_coordination_consent_signed DATE,
    date_home_visit_consent_signed DATE,
    signed_up_on DATE DEFAULT CURRENT_DATE,
    added_to_db_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_synced_with_redcap TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS participant_demographics(
    -- serial 64 auto incrementing
    id serial PRIMARY KEY,
    participant_id integer NOT NULL,
    -- Relates to participants table
        CONSTRAINT FK_participant_demographics_participant_id
            FOREIGN KEY (participant_id)
            REFERENCES participants(id)
            ON DELETE CASCADE,
    age smallint,
    gender TEXT,
    race VARCHAR(255),
    race_other TEXT,
    race_multiple TEXT,
    ethnicity VARCHAR(255),
    language TEXT,
    is_veteran BOOLEAN,
    health_insurance TEXT[],
    highest_education_level VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS participant_health_overview(
    -- serial 64 auto incrementing
    id serial PRIMARY KEY,
    participant_id INTEGER NOT NULL,
    -- Relates to participants table
        CONSTRAINT FK_participant_health_overview_participant_id
            FOREIGN KEY (participant_id)
            REFERENCES participants(id)
            ON DELETE CASCADE,
    height integer,
    reported_health_conditions TEXT,
    allergies TEXT,
    mobility_devices TEXT[],
    has_blood_pressure_cuff BOOLEAN,
    takes_more_than_5_medications BOOLEAN
);

CREATE TABLE IF NOT EXISTS participant_medications(
    id serial PRIMARY KEY,
    participant_id INTEGER NOT NULL,
    -- Relates to participants table
        CONSTRAINT FK_participant_medications_participant_id
            FOREIGN KEY (participant_id)
            REFERENCES participants(id)
            ON DELETE CASCADE,
    name VARCHAR(255),
    dosage VARCHAR(255),
    frequency TEXT,
    date_prescribed DATE,
    date_entered_into_system DATE NOT NULL DEFAULT CURRENT_DATE,
    is_current BOOLEAN,
    date_discontinued DATE,
    comments TEXT
);

CREATE TABLE IF NOT EXISTS participant_goals(
    id serial PRIMARY KEY,
    participant_id INTEGER NOT NULL,
    -- Relates to participants table
        CONSTRAINT FK_participant_medical_history_participant_id
            FOREIGN KEY (participant_id)
            REFERENCES participants(id)
            ON DELETE CASCADE,
    goal TEXT NOT NULL,
    is_active BOOLEAN
);

CREATE TABLE IF NOT EXISTS participant_goal_steps(
    id serial PRIMARY KEY,
    participant_id INTEGER NOT NULL,
    -- Relates to participants table
        CONSTRAINT FK_participant_medical_history_participant_id
            FOREIGN KEY (participant_id)
            REFERENCES participants(id)
            ON DELETE CASCADE,
    goal_id INTEGER,
    -- Relates to participant_goals table
        CONSTRAINT FK_participant_goal_steps_goal_id
            FOREIGN KEY (goal_id)
            REFERENCES participant_goals(id)
            ON DELETE SET NULL,
    step TEXT NOT NULL,
    confidence_level smallint,
    date_set DATE,
    date_to_be_completed DATE,
    action_step BOOLEAN
);
