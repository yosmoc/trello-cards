use dotenv::dotenv;
use serde::Deserialize;
use std::env;


#[derive(Deserialize)]
struct Task {
    name: String,
}

fn fetch_tasks(list_id: &str) -> Result<Vec<Task>, reqwest::Error> {
    let api_key = env::var("TRELLO_API_KEY").unwrap();
    let token = env::var("TRELLO_TOKEN").unwrap();
    let url = format!("https://api.trello.com/1/lists/{}/cards?key={}&token={}", list_id, api_key, token);
    let tasks = reqwest::blocking::get(&url)?.json()?;
    Ok(tasks)
}

fn main() -> Result<(), reqwest::Error>{
    dotenv().ok();
    let list_id = &env::var("TRELLO_LIST_ID").unwrap()[..];
    let tasks = fetch_tasks(list_id)?;
    for task in tasks {
        println!("- {}", task.name);
    }
    Ok(())
}
