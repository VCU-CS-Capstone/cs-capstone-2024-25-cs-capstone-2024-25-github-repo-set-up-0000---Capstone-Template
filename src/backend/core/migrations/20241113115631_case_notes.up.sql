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

CREATE TABLE IF NOT EXISTS case_note_question_answers(
    id serial PRIMARY KEY,
    case_note_id integer NOT NULL,
        CONSTRAINT FK_case_note_question_answers_case_note_id
            FOREIGN KEY (case_note_id)
            REFERENCES case_notes(id)
            ON UPDATE CASCADE
            ON DELETE CASCADE,
    question_id integer NOT NULL,
        CONSTRAINT FK_case_note_question_answers_question_id
            FOREIGN KEY (question_id)
            REFERENCES questions(id)
            ON UPDATE CASCADE
            ON DELETE CASCADE,
    value_radio INTEGER,
        CONSTRAINT FK_case_note_question_answers_value_radio
            FOREIGN KEY (value_radio)
            REFERENCES question_options(id)
            ON UPDATE CASCADE
            ON DELETE SET NULL,
    value_text TEXT,
    value_number INTEGER,
    value_boolean BOOLEAN
);

CREATE TABLE IF NOT EXISTS question_answer_multi_check_box(
    id serial PRIMARY KEY,
    question_answers_id integer NOT NULL,
        CONSTRAINT FK_question_answer_multi_check_box_question_answers_id
            FOREIGN KEY (question_answers_id)
            REFERENCES case_note_question_answers(id)
            ON UPDATE CASCADE
            ON DELETE CASCADE,
    option_id integer NOT NULL,
        CONSTRAINT FK_question_answer_multi_check_box_option_id
            FOREIGN KEY (option_id)
            REFERENCES question_options(id)
            ON UPDATE CASCADE
            ON DELETE CASCADE
);