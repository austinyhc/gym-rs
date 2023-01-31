extern crate reqwest;
extern crate serde_json;

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
const PROG_LANG: &str = "rust";


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
    #[serde(rename = "codeDefinition")]
    pub code_definition: CodeDefinition,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    pub difficulty: String,
    pub question_id: u32,
    pub return_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    operation_name: String,
    variables: serde_json::Value,
    query: String
}

impl Query {
    fn make_question_query(title_slug: &str) -> Query {
        Query {
            operation_name : QUESTION_QUERY_OPERATION.into(),
            variables      : json!({"titleSlug": title_slug}),
            query          : QUESTION_QUERY_STRING.into()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RawResponse {
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

            let client = reqwest::blocking::Client::new();
            let question_title = problem.stat.question_title_slug.as_ref().unwrap();
            let query = Query::make_question_query(&question_title);

            let resp  = client
                .post(URL_GRAPHQL)
                .json(&query)
                .send()
                .unwrap()
                .json::<RawResponse>()
                .unwrap();

            let code_defs = serde_json::from_str::<Vec<CodeDefinition>>
                (&resp.data.question.code_definition).unwrap();

            let code_definition = code_defs.iter().find
                (|e| e.value == PROG_LANG.to_string()).unwrap();

            let meta_data = serde_json::from_str::<serde_json::Value>
                (&resp.data.question.meta_data).unwrap();

            return Some(Problem {
                title: problem.stat.question_title.clone().unwrap(),
                title_slug: problem.stat.question_title_slug.clone().unwrap(),
                content: resp.data.question.content,
                code_definition: code_definition.clone(),
                sample_test_case: resp.data.question.sample_test_case,
                difficulty:  problem.difficulty.level.to_string(),
                question_id: problem.stat.frontend_question_id,
                return_type: meta_data["return"]["type"].to_string().replace("\"", ""),
            });
        }
    }
    None
}

pub fn get_all_problems() -> Result<Problems, Box<reqwest::Error>> {
    Ok(reqwest::blocking::get(URL_PROBLEMS)?.json::<Problems>()?)
}
