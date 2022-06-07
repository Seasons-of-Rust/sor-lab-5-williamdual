use personnel::AstronautJob;
use personnel::Candidate;
fn main() {
    let mut candidates: Vec<Candidate> = Candidate::load_candidate_file();
    candidates.sort_by_key(get_canidate_score); //yo does it auto pass itself? awesome
}

fn get_canidate_score(canidate: &Candidate) -> u32 {
    ((get_job_score(canidate) + canidate.health as u32) * canidate.age as u32) % 3928
}

fn get_job_score(canidate: &Candidate) -> u32 {
    match &canidate.secondary_job {
        Some(secondary_job) => {
            (get_job_code(&canidate.primary_job) * get_job_code(&secondary_job)) % 576
        }
        None => (get_job_code(&canidate.primary_job) * get_job_code(&canidate.primary_job)) % 576,
    }
    //used if statments but couldnt get it to work, so i used match! are you proud of me?
}

fn get_job_code(job: &AstronautJob) -> u32 {
    match job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}
#[test]
fn test_job_code() {
    assert_eq!(251, get_job_code(&AstronautJob::Biogeochemist));
    assert_eq!(281, get_job_code(&AstronautJob::RoverOp));
    assert_eq!(283, get_job_code(&AstronautJob::Scientist));
}
#[test]
fn test_canidate_score() {
    assert_eq!(
        1_066,
        get_canidate_score(&Candidate {
            primary_job: AstronautJob::Biogeochemist,
            secondary_job: std::option::Option::None,
            age: (22),
            health: (10)
        })
    );
    assert_eq!(
        918,
        get_canidate_score(&Candidate {
            primary_job: AstronautJob::RoverOp,
            secondary_job: Some(AstronautJob::Scientist),
            age: (18),
            health: (16)
        })
    );
}
