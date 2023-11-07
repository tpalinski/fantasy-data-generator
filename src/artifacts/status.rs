use rand::Rng;

pub enum ArtifactStatus {
    UNIDENTIFIED,
    IDENTIFIED,
    STOLEN,
    SOLD,
    DAMAGED,
}

impl ArtifactStatus {
    pub fn to_string(&self) -> String {
        match self {
            ArtifactStatus::SOLD => String::from("Sold"),
            ArtifactStatus::STOLEN => String::from("Stolen"),
            ArtifactStatus::DAMAGED => String::from("Damaged"),
            ArtifactStatus::IDENTIFIED => String::from("Identified"),
            ArtifactStatus::UNIDENTIFIED => String::from("Unidentified")
        }
    }

    pub fn all_cases() -> Vec<Self> {
        vec![ArtifactStatus::UNIDENTIFIED, ArtifactStatus::IDENTIFIED, ArtifactStatus::STOLEN, ArtifactStatus::SOLD, ArtifactStatus::DAMAGED]
    }

    pub fn get_random_status() -> String {
        const CASES: usize = 5;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..CASES);
        ArtifactStatus::all_cases()[index].to_string()
    }
}
