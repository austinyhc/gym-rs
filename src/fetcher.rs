extern crate reqwest;
extern crate serde_json;

use serde_json::Value;

const URL_PROBLEMS: &str = "https://leetcode.com/api/problems/algorithms/";
const URL_GRAPHQL: &str = "https://leetcode.com/graphql";
const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        metaData
    }
}"#;
const QUESTION_QUERY_OPERATION: &str = "questionData";


#[derive(Debug, Serialize, Deserialize)]
pub struct Problems {
    user_name: String,
    num_solved: u32,
    num_total: u32,
    ac_easy: u32,
    ac_medium: u32,
    ac_hard: u32,
    pub stat_status_pairs: Vec<StatWithStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatWithStatus {
    pub stat: Stat,
    difficulty: Difficulty,
    paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Difficulty {
    level: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    pub frontend_question_id: u32,
    is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub title: String,
    pub title_slug: String,
    pub content: String,
    pub codeDefinition: Vec<CodeDefinition>,
    pub sampleTestCase: String,
    pub difficulty: String,
    pub question_id: u32,
    pub return_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeDefinition {
    //...
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    operation_name: String,
    variables: serde_json::Value,
    query: String
}

impl Query {
    fn question_query(title_slug: &str) -> Query {
        Query {
            operation_name : QUESTION_QUERY_OPERATION.into(),
            variables      : json!({"titleSlug": title_slug}),
            query          : QUESTION_QUERY_STRING.into()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RawProblem {
    data: Data
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    content: String,
    stats: String,
    #[serde(rename = "codeDefinition")]
    code_definition: String,
    #[serde(rename = "sampleTestCase")]
    sample_test_case: String,
    #[serde(rename = "metaData")]
    meta_data: String,
}

pub fn get_problem(frontend_question_id: &u32) -> Option<Problem> {
    let problems = get_all_problems().unwrap();

    for problem in problems.stat_status_pairs.iter() {
        if problem.stat.frontend_question_id == *frontend_question_id {
            if problem.paid_only { return None; }
        }

        let client = reqwest::blocking::Client::new();
        let resp: RawProblem = client
            .post(URL_GRAPHQL)
            .json(&Query::question_query(
                    problem.stat.question_title_slug.as_ref().unwrap()
            ))
            .send()
            .unwrap()
            .json::<RawProblem>()
            .unwrap();

        println!("{resp:?}");
    }
    None
}

pub fn get_all_problems() -> Result<Problems, Box<reqwest::Error>> {
    Ok(reqwest::blocking::get(URL_PROBLEMS)?.json::<Problems>()?)
}
