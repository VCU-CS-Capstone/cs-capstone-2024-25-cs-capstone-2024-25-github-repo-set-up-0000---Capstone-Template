-- Participants Table.
CREATE TABLE IF NOT EXISTS participants(
    -- bigserial 64 auto incrementing
    id bigserial PRIMARY KEY,
    red_cap_id bigint,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    -- Contact Info
    phone_number_one VARCHAR(255) NOT NULL,
    phone_number_two VARCHAR(255) NOT NULL,
    other_contact TEXT NOT NULL,
    -- Other Info
    program VARCHAR(255) NOT NULL,
    -- Relates to location table
    location INTEGER,
        CONSTRAINT FK_participants_location
            FOREIGN KEY (location)
            REFERENCES locations(id)
            ON DELETE SET NULL,
    status VARCHAR(255) NOT NULL,
    behavioral_risks_identified TEXT,
    date_care_coordination_consent_signed DATE,
    date_home_visit_consent_signed DATE,
    date_signed_up DATE,
    added_to_db_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_synced_with_redcap TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS participant_demographics(
    -- bigserial 64 auto incrementing
    id bigserial PRIMARY KEY,
    participant_id bigint NOT NULL,
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
    -- bigserial 64 auto incrementing
    id bigserial PRIMARY KEY,
    participant_id bigint NOT NULL,
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
    id bigserial PRIMARY KEY,
    participant_id bigint NOT NULL,
    -- Relates to participants table
        CONSTRAINT FK_participant_medications_participant_id
            FOREIGN KEY (participant_id)
            REFERENCES participants(id)
            ON DELETE CASCADE,
    medication_name VARCHAR(255),
    medication_dosage VARCHAR(255),
    date_prescribed DATE,
    date_entered_into_system DATE,
    is_current BOOLEAN,
    date_discontinued DATE,
    comments TEXT
);

CREATE TABLE IF NOT EXISTS participant_goals(
    id bigserial PRIMARY KEY,
    participant_id bigint NOT NULL,
    -- Relates to participants table
        CONSTRAINT FK_participant_medical_history_participant_id
            FOREIGN KEY (participant_id)
            REFERENCES participants(id)
            ON DELETE CASCADE,
    goal TEXT NOT NULL,
    is_active BOOLEAN,
);

CREATE TABLE IF NOT EXISTS participant_goal_steps(
    id bigserial PRIMARY KEY,
    participant_id bigint NOT NULL,
    -- Relates to participants table
        CONSTRAINT FK_participant_medical_history_participant_id
            FOREIGN KEY (participant_id)
            REFERENCES participants(id)
            ON DELETE CASCADE,
    goal_id bigint,
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