-- Add up migration script here
CREATE TABLE IF NOT EXISTS case_notes(
    id serial PRIMARY KEY,
    participant_id integer NOT NULL,
        CONSTRAINT FK_participant_case_notes_participant_id
            FOREIGN KEY (participant_id)
            REFERENCES participants(id)
            ON UPDATE CASCADE
            ON DELETE CASCADE,
    location integer,
        CONSTRAINT FK_participant_case_notes_location
            FOREIGN KEY (location)
            REFERENCES locations(id)
            ON UPDATE CASCADE
            ON DELETE SET NULL,
    visit_type VARCHAR(255),
    age smallint NOT NULL,
    reason_for_visit TEXT,
    info_provided_by_caregiver TEXT,
    date_of_visit DATE NOT NULL,
    pushed_to_redcap BOOLEAN,
    redcap_instance integer,
    last_synced_with_redcap TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS case_note_health_measures(
    id serial PRIMARY KEY,
    case_note_id integer NOT NULL,
        CONSTRAINT FK_case_note_health_measures_case_note_id
            FOREIGN KEY (case_note_id)
            REFERENCES case_notes(id)
            ON UPDATE CASCADE
            ON DELETE CASCADE,
    blood_pressure_sit_systolic smallint,
    blood_pressure_sit_diastolic smallint,
        CONSTRAINT CK_blood_pressure_sit
            CHECK (
                (blood_pressure_sit_systolic is not null) = (blood_pressure_sit_diastolic is not null)
            ),
    blood_pressure_stand_systolic smallint,
    blood_pressure_stand_diastolic smallint,
        CONSTRAINT CK_blood_pressure_stand
            CHECK (
                (blood_pressure_stand_systolic is not null) = (blood_pressure_stand_diastolic is not null)
            ),
    weight real,
    glucose_tested BOOLEAN NOT NULL DEFAULT FALSE,
    glucose_result real,
    fasted_atleast_2_hours BOOLEAN NOT NULL DEFAULT FALSE,
    other TEXT
);

CREATE TABLE IF NOT EXISTS case_note_other_health_visits(
    id serial PRIMARY KEY,
    case_note_id integer NOT NULL,
        CONSTRAINT FK_case_note_other_health_visits_case_note_id
            FOREIGN KEY (case_note_id)
            REFERENCES case_notes(id)
            ON UPDATE CASCADE
            ON DELETE CASCADE,
    emergency_number_called BOOLEAN NOT NULL DEFAULT FALSE,
    refused_ambulance BOOLEAN,
    reason_for_call TEXT,
    last_see_pcp TEXT,
    did_visit_hospital BOOLEAN,
    hositpal_visit TEXT,
    did_visit_ed BOOLEAN,
    ed_visit TEXT
);

CREATE TABLE IF NOT EXISTS case_note_medications(
    id serial PRIMARY KEY,
    case_note_id integer NOT NULL,
        CONSTRAINT FK_case_note_medications_case_note_id
            FOREIGN KEY (case_note_id)
            REFERENCES case_notes(id)
            ON UPDATE CASCADE
            ON DELETE CASCADE,
    has_medications_changed_since_last_visit BOOLEAN,
    new_medication_is_opioid BOOLEAN,
    confirmed_medication_list BOOLEAN,
    has_discrepancies BOOLEAN,
    disrepancies TEXT,
    medication_adherence TEXT,
    did_provide_medication_education BOOLEAN,
    provided_medication_education TEXT,
    provided_medication_list BOOLEAN,
    did_provide_medication_adherence_assistance BOOLEAN,
    provided_medication_adherence_assistance TEXT,
    was_pharmacy_contacted BOOLEAN,
    pharmacy_contacted TEXT,
    was_pcp_contacted BOOLEAN,
    pcp_contacted TEXT
);