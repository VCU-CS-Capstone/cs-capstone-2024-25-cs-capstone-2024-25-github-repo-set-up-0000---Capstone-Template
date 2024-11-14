use std::{collections::HashMap, path::Path};

use chrono::Local;
use cs25_303_core::{
    database::red_cap::{
        case_notes::new::{NewBloodPressure, NewCaseNote, NewCaseNoteHealthMeasures},
        locations::Locations,
        participants::{
            goals::{NewParticipantGoal, NewParticipantGoalsSteps},
            NewDemographics, NewHealthOverview, NewMedication, NewParticipant, Participants,
        },
    },
    red_cap_data::{
        Gender, HealthInsurance, MedicationFrequency, Programs, Race, Status, VisitType,
    },
};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sqlx::{types::chrono::NaiveDate, PgPool};
use tracing::error;
/// Notes we will use for data generation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParticipantExtendedInfo {
    pub has_high_blood_pressure: bool,
    pub has_diabetes: bool,
}
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RandomDateOptions {
    pub min: Option<NaiveDate>,
    pub max: Option<NaiveDate>,
}
impl RandomDateOptions {
    pub fn random_date(&self) -> NaiveDate {
        let Self { min, max } = self;
        let min = min.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        let max = max.unwrap_or_else(|| Local::now().date_naive());
        let days = max.signed_duration_since(min).num_days();
        let random_days = rand::thread_rng().gen_range(0..days);
        min + chrono::Duration::days(random_days)
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "random", content = "options")]
pub enum RandomValue {
    Array(Vec<Value>),
    Number { min: i32, max: i32 },
    Bool,
    Date(Option<RandomDateOptions>),
}
impl RandomValue {
    pub fn random_string_from_options(&self) -> String {
        match self {
            RandomValue::Array(options) => {
                let value = options.choose(&mut rand::thread_rng()).unwrap();
                value.as_str().unwrap_or_default().to_owned()
            }
            RandomValue::Number { min, max } => {
                rand::thread_rng().gen_range(*min..*max).to_string()
            }
            RandomValue::Bool => rand::thread_rng().gen_bool(0.5).to_string(),
            RandomValue::Date(value) => {
                let value = value.clone().unwrap_or_default();
                value.random_date().to_string()
            }
        }
    }
    pub fn date(&self) -> NaiveDate {
        match self {
            RandomValue::Date(value) => {
                let value = value.clone().unwrap_or_default();
                value.random_date()
            }
            _ => {
                error!("Not a date");
                Local::now().date_naive()
            }
        }
    }
    pub fn random_i16(&self) -> i16 {
        match self {
            RandomValue::Number { min, max } => rand::thread_rng().gen_range(*min..*max) as i16,
            _ => {
                error!("Not a number");
                rand::thread_rng().gen_range(0..100) as i16
            }
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ValueOrRandom<T> {
    Value(T),
    Random(RandomValue),
}
impl ValueOrRandom<i16> {
    pub fn i16_value(&self) -> i16 {
        match self {
            ValueOrRandom::Value(value) => *value,
            ValueOrRandom::Random(random_value) => random_value.random_i16(),
        }
    }
}
impl ValueOrRandom<String> {
    pub fn string_value(&self) -> String {
        match self {
            ValueOrRandom::Value(value) => value.clone(),
            ValueOrRandom::Random(random_value) => random_value.random_string_from_options(),
        }
    }
}
impl ValueOrRandom<NaiveDate> {
    pub fn date_value(&self) -> NaiveDate {
        match self {
            ValueOrRandom::Value(value) => *value,
            ValueOrRandom::Random(random_value) => random_value.date(),
        }
    }
}
impl<T> ValueOrRandom<T>
where
    T: From<String> + Clone,
{
    pub fn value_from_string(&self) -> T {
        match self {
            ValueOrRandom::Value(value) => value.clone(),
            ValueOrRandom::Random(random_value) => {
                let string_value = random_value.random_string_from_options();
                T::from(string_value)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RandomSets {
    pub rand: rand::rngs::StdRng,
    // TODO: Share a random generator
    pub participants: Vec<RandomParticipant>,
    pub goals: Vec<RandomCompleteGoal>,
    pub medications: Vec<RandomMedication>,
    pub behbehavioral_risks_identified: Vec<String>,
    pub r_locations: Vec<Locations>,
    pub m_locations: Vec<Locations>,
    pub reasons_for_visit: Vec<String>,
    pub info_provided_by_caregiver: Vec<String>,
    pub extended_patient_info: HashMap<i32, ParticipantExtendedInfo>,
}
impl RandomSets {
    pub fn randon_behavioral_risks_identified(&mut self) -> Option<String> {
        Some(
            self.behbehavioral_risks_identified
                .choose(&mut self.rand)
                .unwrap()
                .clone(),
        )
    }
    pub fn random_health_overview(&mut self) -> NewHealthOverview {
        let height = match self.rand.gen_range(0..100) {
            0..50 => None,
            50..75 => Some(rand::thread_rng().gen_range(50..100)),
            _ => Some(rand::thread_rng().gen_range(100..300)),
        };
        NewHealthOverview {
            height,
            ..Default::default()
        }
    }
    pub fn random_demographics(&mut self, gender: Gender) -> NewDemographics {
        let is_veteran = !matches!(self.rand.gen_range(0..100), 0..90);
        let (race, race_other, race_multiple) = match rand::thread_rng().gen_range(0..100) {
            0..50 => (Some(Race::White), None, None),
            50..65 => (Some(Race::Black), None, None),
            65..70 => (Some(Race::Hispanic), None, None),
            70..90 => (None, Some("Other".to_string()), None),
            _ => (
                Some(Race::Multiracial),
                None,
                Some("White, Black".to_string()),
            ),
        };
        let health_insurance = match rand::thread_rng().gen_range(0..100) {
            0..50 => vec![HealthInsurance::Medicaid],
            50..75 => vec![HealthInsurance::Medicare],
            75..90 => vec![HealthInsurance::Private],
            _ => vec![],
        };
        NewDemographics {
            age: Some(self.rand.gen_range(18..85) as i16),
            gender: Some(gender),
            is_veteran: Some(is_veteran),
            race,
            race_other,
            race_multiple,
            health_insurance,
            ..Default::default()
        }
    }
    pub fn random_medications(&mut self) -> Vec<NewMedication> {
        let number_of_meds = self.rand.gen_range(0..10);

        let mut meds = Vec::with_capacity(number_of_meds);

        for _ in 0..number_of_meds {
            let random = self.medications.choose(&mut self.rand).unwrap();
            // TODO: Prevent duplicates
            meds.push(random.create_new_medication());
        }
        meds
    }
    pub fn random_goals(&mut self) -> Vec<(NewParticipantGoal, Vec<NewParticipantGoalsSteps>)> {
        let number_of_meds = self.rand.gen_range(0..3);
        let mut goals = Vec::with_capacity(number_of_meds);
        for _ in 0..number_of_meds {
            let random = self.goals.choose(&mut self.rand).unwrap();
            goals.push(random.create_new_goal());
        }
        goals
    }
    pub fn pick_random_program(&mut self) -> Programs {
        if self.rand.gen_bool(1f64 / 3f64) {
            Programs::MHWP
        } else {
            Programs::RHWP
        }
    }
    pub fn location_for_program(&mut self, program: Programs) -> Locations {
        if program == Programs::MHWP {
            self.m_locations
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone()
        } else {
            self.r_locations
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone()
        }
    }
    pub fn random_info_by_caregiver(&mut self) -> Option<String> {
        // 50 chance of none
        if self.rand.gen_bool(0.5) {
            return None;
        }
        Some(
            self.info_provided_by_caregiver
                .choose(&mut self.rand)
                .unwrap()
                .clone(),
        )
    }
    pub fn random_reason_for_visit(&mut self) -> Option<String> {
        // 25 chance of none
        if self.rand_bool(0.25) {
            return None;
        }
        Some(
            self.reasons_for_visit
                .choose(&mut self.rand)
                .unwrap()
                .clone(),
        )
    }
    pub fn random_visit_type(&self) -> Option<VisitType> {
        match rand::thread_rng().gen_range(0..100) {
            0..10 => Some(VisitType::OnsiteAndHome),
            _ => Some(VisitType::Onsite),
        }
    }
    // TODO: Add Standing blood pressure
    pub fn random_blood_pressure(
        &mut self,
        participant: i32,
    ) -> (Option<NewBloodPressure>, Option<NewBloodPressure>) {
        // About 47% chance of having high blood pressure

        if self.extended_patient_info[&participant].has_high_blood_pressure {
            (
                Some(NewBloodPressure {
                    systolic: self.rand.gen_range(130..180) as i16,
                    diastolic: self.rand.gen_range(80..120) as i16,
                }),
                None,
            )
        } else {
            (
                Some(NewBloodPressure {
                    systolic: self.rand.gen_range(90..120) as i16,
                    diastolic: self.rand.gen_range(60..80) as i16,
                }),
                None,
            )
        }
    }
    fn rand_bool(&mut self, chance: f64) -> bool {
        self.rand.gen_bool(chance)
    }
    pub fn create_extended_profile_for_partiicpant(&mut self, participant: i32) {
        let has_high_blood_pressure = self.rand_bool(0.47);
        let has_diabetes = self.rand_bool(0.1);

        self.extended_patient_info.insert(
            participant,
            ParticipantExtendedInfo {
                has_high_blood_pressure,
                has_diabetes,
            },
        );
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RandomParticipant {
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
}
impl RandomParticipant {
    pub fn random_phone_number() -> String {
        let mut rng = rand::thread_rng();
        let phone_number: String = format!(
            "(555) {:03}-{:04}",
            rng.gen_range(100..999),
            rng.gen_range(1000..9999)
        );
        phone_number
    }
    pub fn random_status() -> Status {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..100) {
            0..75 => Status::Active,
            75..85 => Status::Inactive,
            85..95 => Status::NoValidContactStatus,
            _ => Status::Deceases,
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RandomCompleteGoal {
    pub goal: RandomGoal,
    pub steps: Vec<RandomGoalStep>,
}
impl RandomCompleteGoal {
    pub fn create_new_goal(&self) -> (NewParticipantGoal, Vec<NewParticipantGoalsSteps>) {
        let goal = self.goal.create_new_goal();

        let random_step: &RandomGoalStep = self.steps.choose(&mut rand::thread_rng()).unwrap();
        let steps = vec![random_step.create_new_goal_step()];
        (goal, steps)
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RandomGoal {
    pub goal: String,
    pub is_active: bool,
}
impl RandomGoal {
    pub fn create_new_goal(&self) -> NewParticipantGoal {
        let RandomGoal { goal, is_active } = self;
        NewParticipantGoal {
            goal: goal.clone(),
            is_active: Some(*is_active),
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RandomGoalStep {
    pub step: String,
    pub confidence_in_achieving: ValueOrRandom<i16>,
    pub date_set: ValueOrRandom<NaiveDate>,
    pub date_to_be_achieved: ValueOrRandom<NaiveDate>,
}
impl RandomGoalStep {
    pub fn create_new_goal_step(&self) -> NewParticipantGoalsSteps {
        let RandomGoalStep {
            step,
            confidence_in_achieving,
            date_set,
            date_to_be_achieved,
        } = self;
        let confidence_in_achieving = confidence_in_achieving.i16_value();
        let date_set = date_set.date_value();
        let date_to_be_achieved = date_to_be_achieved.date_value();
        NewParticipantGoalsSteps {
            goal_id: None,
            step: step.clone(),
            confidence_level: Some(confidence_in_achieving),
            date_set: Some(date_set),
            date_to_be_completed: Some(date_to_be_achieved),
            ..Default::default()
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RandomMedication {
    pub name: String,
    pub dosage: ValueOrRandom<String>,
    pub frequency: ValueOrRandom<MedicationFrequency>,
    pub start_date: Option<ValueOrRandom<NaiveDate>>,
}
impl RandomMedication {
    pub fn create_new_medication(&self) -> NewMedication {
        let RandomMedication {
            name,
            dosage,
            frequency,
            start_date,
        } = self;

        let dosage = dosage.string_value();
        let freqeuency = frequency.value_from_string();
        let start_date = start_date.as_ref().map(|date| date.date_value());

        NewMedication {
            name: name.clone(),
            dosage,
            frequency: freqeuency,
            date_prescribed: None,
            date_entered_into_system: start_date,
            is_current: None,
            date_discontinued: None,
            comments: None,
        }
    }
}

fn load_random_set<T>(name: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("random")
        .join("sets")
        .join(format!("{}.json", name));
    println!("Loading random set from path: {:?}", path);
    let file = std::fs::read_to_string(path)?;
    let random_sets: T = serde_json::from_str(&file)?;
    Ok(random_sets)
}
pub async fn load_random_sets(database: Option<&PgPool>) -> anyhow::Result<RandomSets> {
    let random_medication: Vec<RandomMedication> = load_random_set("medications")?;
    let random_participants: Vec<RandomParticipant> = load_random_set("participants")?;
    let random_goals: Vec<RandomCompleteGoal> = load_random_set("goals")?;
    let random_behavioral_risks_identified: Vec<String> =
        load_random_set("behavioral_risks_identified")?;
    let reasons_for_visit: Vec<String> = load_random_set("reason_for_visit")?;
    let info_provided_by_caregiver: Vec<String> = load_random_set("info_by_caregiver")?;
    let (r_locations, m_locations) = if let Some(database) = database {
        (
            Locations::find_all_in_program(Programs::RHWP, database).await?,
            Locations::find_all_in_program(Programs::MHWP, database).await?,
        )
    } else {
        (vec![], vec![])
    };

    Ok(RandomSets {
        rand: rand::rngs::StdRng::from_entropy(),
        participants: random_participants,
        goals: random_goals,
        medications: random_medication,
        behbehavioral_risks_identified: random_behavioral_risks_identified,
        r_locations,
        m_locations,
        reasons_for_visit,
        info_provided_by_caregiver,
        extended_patient_info: HashMap::new(),
    })
}

pub async fn generate_participants(count: usize, database: PgPool) -> anyhow::Result<()> {
    let mut random_sets = load_random_sets(Some(&database)).await?;

    for _ in 0..count {
        let RandomParticipant {
            first_name,
            last_name,
            gender,
        } = random_sets
            .participants
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        let program_and_location = random_sets.pick_random_program();
        let location = random_sets.location_for_program(program_and_location);

        let new_participant = NewParticipant {
            first_name,
            last_name,
            red_cap_id: None,
            phone_number_one: Some(RandomParticipant::random_phone_number()),
            phone_number_two: None,
            other_contact: None,
            program: program_and_location,
            location: Some(location.id),
            status: Some(RandomParticipant::random_status()),
            behavioral_risks_identified: random_sets.randon_behavioral_risks_identified(),
            date_care_coordination_consent_signed: None,
            date_home_visit_consent_signed: None,
            signed_up_on: Local::now().date_naive(),
            last_synced_with_redcap: None,
        };
        let part = new_participant.insert_return_participant(&database).await?;
        random_sets.create_extended_profile_for_partiicpant(part.id);
        let health_overview = random_sets.random_health_overview();
        health_overview.insert_none(part.id, &database).await?;

        let demographics = random_sets.random_demographics(gender);

        demographics.insert_none(part.id, &database).await?;

        let medications = random_sets.random_medications();

        for medication in medications {
            medication.insert_none(part.id, &database).await?;
        }

        let goals = random_sets.random_goals();

        for (goal, steps) in goals {
            let goal = goal.insert_return_goal(part.id, &database).await?;
            for step in steps {
                step.insert_with_goal_return_none(part.id, goal.id, &database)
                    .await?;
            }
        }
        let number_of_case_notes = rand::thread_rng().gen_range(0..10);
        let current_date = Local::now().date_naive();
        for _ in 0..number_of_case_notes {
            let date_of_visit = current_date - chrono::Duration::weeks(1);
            generate_random_case_note_on(&mut random_sets, part.clone(), date_of_visit, &database)
                .await?;
        }
    }
    Ok(())
}

async fn generate_random_case_note_on(
    random: &mut RandomSets,
    participant: Participants,
    date_of_visit: NaiveDate,
    database: &PgPool,
) -> anyhow::Result<()> {
    let visit_type = random.random_visit_type();
    let reason_for_visit = random.random_reason_for_visit();
    let info_provided_by_caregiver = random.random_info_by_caregiver();

    let new_case_note = NewCaseNote {
        location: participant.location,
        visit_type,
        age: 0, // TODO: Pass Age Into this function
        reason_for_visit,
        info_provided_by_caregiver,
        date_of_visit,
        ..Default::default()
    };
    let case_note = new_case_note
        .insert_return_case_note(participant.id, database)
        .await?;
    let (sit, stand) = random.random_blood_pressure(participant.id);
    let new_health_measures = NewCaseNoteHealthMeasures {
        sit,
        stand,
        ..Default::default()
    };

    new_health_measures
        .insert_return_none(case_note.id, database)
        .await?;

    Ok(())
}
#[cfg(test)]

mod tests {

    #[tokio::test]
    pub async fn load_full() -> anyhow::Result<()> {
        let random_sets = super::load_random_sets(None).await?;
        println!("{:#?}", random_sets);
        Ok(())
    }
}
