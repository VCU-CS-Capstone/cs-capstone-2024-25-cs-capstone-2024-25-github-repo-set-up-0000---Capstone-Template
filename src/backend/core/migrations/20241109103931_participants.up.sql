-- Participants Table.
CREATE TABLE IF NOT EXISTS participants(
    -- bigserial 64 auto incrementing
    id bigserial PRIMARY KEY,
    id bigint,
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