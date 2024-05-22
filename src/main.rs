use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand};
use reqwest::header::{self, HeaderMap};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::io::{self, Read};
use log::{debug};
use env_logger;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Block {
    format: Option<String>,
    #[serde(rename = "journal?")]
    journal: Option<bool>,
    uuid: Option<String>,
    page: Option<Page>,
    id: Option<i64>,
    #[serde(rename = "journalDay")]
    journal_day: Option<i64>,
    parent: Option<Parent>,
    #[serde(default)]
    children: Vec<Vec<serde_json::Value>>,
    #[serde(default)]
    properties: HashMap<String, serde_json::Value>,
    warning: Option<String>,
    #[serde(default)]
    #[serde(rename = "PathRefs")]
    path_refs: Vec<PathRef>,
    content: Option<String>,
    #[serde(default)]
    #[serde(rename = "propertiesOrder")]
    properties_order: Vec<serde_json::Value>,
    left: Option<Left>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Page {
    id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Parent {
    id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PathRef {
    id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Left {
    id: Option<i64>,
}

#[derive(Parser)]
#[command(name = "rlu")]
#[command(about = "Rust Logseq Utility")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        date: Option<String>,
    },
    Show {
        #[arg(long)]
        date: String,
    },
    Get {
        #[arg(long)]
        entry_id: String,
        #[arg(long)]
        date: Option<String>,
    },
    OutputContent {
        #[arg(long)]
        entry_id: String,
        #[arg(long)]
        date: Option<String>,
    },
    AddToStart {
        #[arg(long)]
        entry_id: String,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        date: Option<String>,
    },
    AppendToEnd {
        #[arg(long)]
        entry_id: String,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        date: Option<String>,
    },
    AddChildNode {
        #[arg(long)]
        entry_id: String,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        date: Option<String>,
    },
    Delete {
        #[arg(long)]
        entry_id: String,
        #[arg(long)]
        date: Option<String>,
    },
}

pub struct Client {
    client: reqwest::blocking::Client,
    current_journal: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .default_headers(Self::client_headers())
                .build()
                .unwrap(),
            current_journal: None,
        }
    }

    fn client_headers() -> HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&env::var("LOGSEQ_API_KEY").unwrap()).unwrap(),
        );
        headers
    }

    pub fn add_journal_note_from_stdin(&mut self, _date: Option<String>) {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content).unwrap();
        debug!("Content from stdin: {}", content);
        self.add_journal_note(&content);
    }

    pub fn add_journal_note_from_flag(&mut self, content: &str, _date: Option<String>) {
        debug!("Content from flag: {}", content);
        self.add_journal_note(content);
    }

    fn read_content(&self, input_content: Option<String>) -> String {
        match input_content {
            Some(content) => content,
            None => {
                let mut content = String::new();
                io::stdin().read_to_string(&mut content).unwrap();
                content
            }
        }
    }

    fn add_journal_note(&mut self, note_text: &str) {
        let journal_id = self.current_journal();
        debug!("Journal ID: {}", journal_id);

        let mut lines = note_text.lines();
        if let Some(first_line) = lines.next() {
            let body = json!({
                "method": "logseq.Editor.insertBlock",
                "args": [
                    journal_id,
                    first_line,
                    {"isPageBlock": true}
                ]
            });

            debug!("Request body: {}", body);

            let res = self.client.post(api_url()).json(&body).send();

            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        debug!("Task added to journal!");

                        if let Some(block_id) = response.json::<HashMap<String, serde_json::Value>>()
                            .unwrap().get("uuid").and_then(|v| v.as_str()) {

                            // Process the remaining lines
                            let mut current_parent_id = block_id.to_string();
                            let mut stack = vec![current_parent_id.clone()];

                            for line in lines {
                                let current_level = line.chars().take_while(|&c| c == '#').count();
                                if current_level > 0 {
                                    while stack.len() > current_level {
                                        stack.pop();
                                    }
                                    current_parent_id = stack.last().cloned().unwrap_or_else(|| block_id.to_string());
                                } else if line.starts_with("- ") {
                                    current_parent_id = stack.last().cloned().unwrap_or_else(|| block_id.to_string());
                                }

                                let body = json!({
                                    "method": "logseq.Editor.insertBlock",
                                    "args": [
                                        current_parent_id,
                                        line,
                                        {"isPageBlock": false}
                                    ]
                                });

                                let res = self.client.post(api_url()).json(&body).send();
                                if let Ok(response) = res {
                                    if response.status().is_success() {
                                        debug!("Sub-block added: {}", line);
                                        if let Some(new_id) = response.json::<HashMap<String, serde_json::Value>>()
                                            .unwrap().get("uuid").and_then(|v| v.as_str()) {
                                            if current_level > 0 {
                                                stack.push(new_id.to_string());
                                            }
                                        }
                                    } else {
                                        debug!("Failed to add sub-block: {:?}", response.text().unwrap());
                                    }
                                } else {
                                    debug!("Error adding sub-block: {:?}", res.unwrap_err());
                                }
                            }
                        }
                    } else {
                        let error_text = response.text().unwrap();
                        debug!("Failed to add task to journal: {:?}", error_text);
                    }
                }
                Err(error) => {
                    debug!("Error: {:?}", error);
                }
            }
        }
    }

    fn current_journal(&mut self) -> String {
        self.current_journal
            .get_or_insert_with(|| get_journal_uuid(&self.client).expect("Could not get journal id"))
            .to_string()
    }

    pub fn show_journal_entries(&self, date: &str) {
        debug!("Showing journal entries for date: {}", date);

        let query_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .expect("Invalid date format")
            .format("%Y%m%d")
            .to_string();

        debug!("Formatted date for query: {}", query_date);

        let query = json!({
            "method": "logseq.DB.datascriptQuery",
            "args": [
                "[:find (pull ?h [*])
                :in $ ?today
                :where
                [?h :block/page ?p]
                [?p :block/journal? true]
                [?p :block/journal-day ?d]
                [(== ?d ?today)] ]",
                query_date
            ]
        });

        debug!("Request body: {}", query);

        let res = self.client.post(api_url()).json(&query).send();

        match res {
            Ok(response) => {
                let raw_response = response.text().unwrap();
                debug!("Raw response: {}", raw_response);

                let entries: Result<Vec<Vec<HashMap<String, serde_json::Value>>>, _> = serde_json::from_str(&raw_response);
                match entries {
                    Ok(entries) => {
                        debug!("Entries: {:?}", entries);
                        for entry_list in entries {
                            for entry in entry_list {
                                if let Some(uuid) = entry.get("uuid").and_then(|v| v.as_str()) {
                                    if let Some(content) = entry.get("content").and_then(|v| v.as_str()) {
                                        if !content.trim().is_empty() {
                                            let preview = content.split_whitespace().take(10).collect::<Vec<_>>().join(" ");
                                            println!("{} {}", uuid, preview);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        debug!("Failed to parse journal entries: {:?}", err);
                    }
                }
            }
            Err(error) => {
                debug!("Error: {:?}", error);
            }
        }
    }

    pub fn get_journal_entry(&self, entry_id: &str, _date: Option<String>) {
        eprintln!("Getting journal entry with ID: {}", entry_id);

        let query = json!({
            "method": "logseq.Editor.getBlock",
            "args": [
                entry_id
            ]
        });

        debug!("Request body: {}", query);

        let res = self.client.post(api_url()).json(&query).send();

        match res {
            Ok( response) => {
                let raw_response = response.text().unwrap();
                debug!("Raw response: {}", raw_response);

                let entry: Result<HashMap<String, serde_json::Value>, _> = serde_json::from_str(&raw_response);
                match entry {
                    Ok(entry) => {
                        println!("{:?}", entry);
                    }
                    Err(err) => {
                        eprintln!("Failed to parse journal entry: {:?}", err);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error: {:?}", error);
            }
        }
    }

    pub fn output_content(&self, entry_id: &str, _date: Option<String>) {
        debug!("Getting content for entry with ID: {}", entry_id);

        // Get the block details by the entry ID
        let query = json!({
            "method": "logseq.Editor.getBlock",
            "args": [
                entry_id
            ]
        });

        let res = self.client.post(api_url()).json(&query).send();

        match res {
            Ok( response) => {
                let raw_response = response.text().unwrap();
                debug!("Raw response: {}", raw_response);

                let entry: Result<Block, _> = serde_json::from_str(&raw_response);
                match entry {
                    Ok(entry) => {
                        let mut content = String::new();
                        self.collect_block_content(&entry, &mut content, 0);
                        println!("{}", content);
                    }
                    Err(err) => {
                        eprintln!("Failed to parse block: {:?}", err);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error: {:?}", error);
            }
        }
    }

    fn collect_block_content(&self, block: &Block, content: &mut String, indent_level: usize) {
        if let Some(block_content) = &block.content {
            content.push_str(&"  ".repeat(indent_level));
            content.push_str(block_content);
            content.push('\n');
        }
        for child in &block.children {
            if let Some(uuid) = child.get(1).and_then(|v| v.as_str()) {
                let child_block = self.get_block_by_uuid(uuid);
                self.collect_block_content(&child_block, content, indent_level + 1);
            }
        }
    }

    fn get_block_by_uuid(&self, uuid: &str) -> Block {
        let query = json!({
            "method": "logseq.Editor.getBlock",
            "args": [
                uuid
            ]
        });

        let res = self.client.post(api_url()).json(&query).send();

        match res {
            Ok( response) => {
                let raw_response = response.text().unwrap();
                debug!("Raw response for child block: {}", raw_response);

                let block: Result<Block, _> = serde_json::from_str(&raw_response);
                match block {
                    Ok(block) => block,
                    Err(err) => {
                        debug!("Failed to parse child block: {:?}", err);
                        Block::default()
                    }
                }
            }
            Err(error) => {
                debug!("Error retrieving child block: {:?}", error);
                Block::default()
            }
        }
    }

    pub fn add_to_start(&mut self, entry_id: &str, input_content: Option<String>, _date: Option<String>) {
        let new_content = self.read_content(input_content);
        debug!("Adding content to the start of entry with ID: {}", entry_id);

        let query = json!({
            "method": "logseq.Editor.getBlock",
            "args": [
                entry_id
            ]
        });

        let res = self.client.post(api_url()).json(&query).send();

        match res {
            Ok( response) => {
                let raw_response = response.text().unwrap();
                debug!("Raw response: {}", raw_response);

                let  entry: Block = serde_json::from_str(&raw_response).unwrap();
                if let Some(content) = entry.content {
                    let updated_content = format!("{} {}", new_content, content);
                    let update_query = json!({
                        "method": "logseq.Editor.updateBlock",
                        "args": [
                            entry_id,
                            updated_content
                        ]
                    });

                    let update_res = self.client.post(api_url()).json(&update_query).send();

                    match update_res {
                        Ok(update_response) => {
                            if update_response.status().is_success() {
                                eprintln!("Content added to the start of the entry.");
                            } else {
                                eprintln!("Failed to update entry: {:?}", update_response.text().unwrap());
                            }
                        }
                        Err(update_error) => {
                            eprintln!("Error: {:?}", update_error);
                        }
                    }
                } else {
                    eprintln!("No content found for this entry.");
                }
            }
            Err(error) => {
                eprintln!("Error: {:?}", error);
            }
        }
    }

    pub fn append_to_end(&mut self, entry_id: &str, input_content: Option<String>, _date: Option<String>) {
        let new_content = self.read_content(input_content);
        debug!("Appending content to the end of entry with ID: {}", entry_id);

        let query = json!({
            "method": "logseq.Editor.getBlock",
            "args": [
                entry_id
            ]
        });

        let res = self.client.post(api_url()).json(&query).send();

        match res {
            Ok( response) => {
                let raw_response = response.text().unwrap();
                debug!("Raw response: {}", raw_response);

                let  entry: Block = serde_json::from_str(&raw_response).unwrap();
                if let Some(content) = entry.content {
                    let updated_content = format!("{} {}", content, new_content);
                    let update_query = json!({
                        "method": "logseq.Editor.updateBlock",
                        "args": [
                            entry_id,
                            updated_content
                        ]
                    });

                    let update_res = self.client.post(api_url()).json(&update_query).send();

                    match update_res {
                        Ok(update_response) => {
                            if update_response.status().is_success() {
                                eprintln!("Content appended to the end of the entry.");
                            } else {
                                eprintln!("Failed to update entry: {:?}", update_response.text().unwrap());
                            }
                        }
                        Err(update_error) => {
                            eprintln!("Error: {:?}", update_error);
                        }
                    }
                } else {
                    eprintln!("No content found for this entry.");
                }
            }
            Err(error) => {
                eprintln!("Error: {:?}", error);
            }
        }
    }

    pub fn add_child_node(&mut self, entry_id: &str, input_content: Option<String>, _date: Option<String>) {
        let new_content = self.read_content(input_content);
        debug!("Adding child node to entry with ID: {}", entry_id);

        self.process_lines_as_children(entry_id, &new_content);
    }

    fn process_lines_as_children(&mut self, parent_id: &str, note_text: &str) {
        let mut lines = note_text.lines();
        if let Some(first_line) = lines.next() {
            let body = json!({
                "method": "logseq.Editor.insertBlock",
                "args": [
                    parent_id,
                    first_line,
                    {"isPageBlock": false}
                ]
            });

            debug!("Request body: {}", body);

            let res = self.client.post(api_url()).json(&body).send();

            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        debug!("Child node added!");

                        if let Some(block_id) = response.json::<HashMap<String, serde_json::Value>>()
                            .unwrap().get("uuid").and_then(|v| v.as_str()) {

                            // Process the remaining lines
                            let mut current_parent_id = block_id.to_string();
                            let mut stack = vec![current_parent_id.clone()];

                            for line in lines {
                                let current_level = line.chars().take_while(|&c| c == '#').count();
                                if current_level > 0 {
                                    while stack.len() > current_level {
                                        stack.pop();
                                    }
                                    current_parent_id = stack.last().cloned().unwrap_or_else(|| block_id.to_string());
                                } else if line.starts_with("- ") {
                                    current_parent_id = stack.last().cloned().unwrap_or_else(|| block_id.to_string());
                                }

                                let body = json!({
                                    "method": "logseq.Editor.insertBlock",
                                    "args": [
                                        current_parent_id,
                                        line,
                                        {"isPageBlock": false}
                                    ]
                                });

                                let res = self.client.post(api_url()).json(&body).send();
                                if let Ok(response) = res {
                                    if response.status().is_success() {
                                        debug!("Sub-block added: {}", line);
                                        if let Some(new_id) = response.json::<HashMap<String, serde_json::Value>>()
                                            .unwrap().get("uuid").and_then(|v| v.as_str()) {
                                            if current_level > 0 {
                                                stack.push(new_id.to_string());
                                            }
                                        }
                                    } else {
                                        debug!("Failed to add sub-block: {:?}", response.text().unwrap());
                                    }
                                } else {
                                    debug!("Error adding sub-block: {:?}", res.unwrap_err());
                                }
                            }
                        }
                    } else {
                        let error_text = response.text().unwrap();
                        debug!("Failed to add child node: {:?}", error_text);
                    }
                }
                Err(error) => {
                    debug!("Error: {:?}", error);
                }
            }
        }
    }

    pub fn delete_entry(&self, entry_id: &str, _date: Option<String>) {
        eprintln!("Deleting entry with ID: {}", entry_id);

        let query = json!({
            "method": "logseq.Editor.removeBlock",
            "args": [
                entry_id
            ]
        });

        let res = self.client.post(api_url()).json(&query).send();

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    eprintln!("Entry deleted.");
                } else {
                    eprintln!("Failed to delete entry: {:?}", response.text().unwrap());
                }
            }
            Err(error) => {
                eprintln!("Error: {:?}", error);
            }
        }
    }

    pub fn get_page_blocks_tree(&self, page_id: &str) -> Vec<Block> {
        let query = json!({
            "method": "logseq.Editor.getPageBlocksTree",
            "args": [
                page_id
            ]
        });

        let res = self.client.post(api_url()).json(&query).send();

        match res {
            Ok( response) => {
                let raw_response = response.text().unwrap();
                debug!("Raw response: {}", raw_response);

                let blocks: Result<Vec<Block>, _> = serde_json::from_str(&raw_response);
                match blocks {
                    Ok(blocks) => blocks,
                    Err(err) => {
                        debug!("Failed to parse page blocks: {:?}", err);
                        Vec::new()
                    }
                }
            }
            Err(error) => {
                debug!("Error: {:?}", error);
                Vec::new()
            }
        }
    }
}

fn get_journal_uuid(client: &reqwest::blocking::Client) -> Result<String, String> {
    let now = Local::now();
    let query = json!({
        "method": "logseq.DB.datascriptQuery",
        "args": [
            "[:find (pull ?p [*])
            :in $ ?today
            :where [?b :block/page ?p]
                   [?p :block/journal? true]
                   [?p :block/journal-day ?d]
                   [(>= ?d ?today)] ]",
        &now.format("%Y%m%d").to_string()
        ]
    });

    debug!("Request body for journal UUID: {}", query);

    let journal_res = client.post(api_url()).json(&query).send();

    match journal_res {
        Ok( response) => {
            let raw_response = response.text().unwrap();
            debug!("Raw response: {}", raw_response);

            let json: Result<Vec<Vec<HashMap<String, serde_json::Value>>>, _> = serde_json::from_str(&raw_response);
            match json {
                Ok(json) => {
                    debug!("Journal response JSON: {:?}", json);
                    let journal_id = json[0][0]["uuid"].as_str().unwrap().to_string();
                    debug!("Journal UUID: {}", journal_id);
                    Ok(journal_id)
                }
                Err(err) => {
                    eprintln!("Failed to parse journal ID: {:?}", err);
                    Err(format!("Failed to parse journal ID: {:?}", err))
                }
            }
        }
        Err(error) => {
            eprintln!("Error getting journal ID: {:?}", error);
            Err(format!("Error: {:?}", error))
        }
    }
}

fn api_url() -> String {
    env::var("LOGSEQ_API_URL").unwrap_or_else(|_| "http://127.0.0.1:12315/api".to_string())
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let mut client = Client::new();

    match &cli.command {
        Commands::Add { content, date } => {
            if let Some(content) = content {
                client.add_journal_note_from_flag(content, date.clone());
            } else {
                client.add_journal_note_from_stdin(date.clone());
            }
        }
        Commands::Show { date } => {
            client.show_journal_entries(date);
        }
        Commands::Get { entry_id, date } => {
            client.get_journal_entry(entry_id, date.clone());
        }
        Commands::OutputContent { entry_id, date } => {
            client.output_content(entry_id, date.clone());
        }
        Commands::AddToStart { entry_id, content, date } => {
            client.add_to_start(entry_id, content.clone(), date.clone());
        }
        Commands::AppendToEnd { entry_id, content, date } => {
            client.append_to_end(entry_id, content.clone(), date.clone());
        }
        Commands::AddChildNode { entry_id, content, date } => {
            client.add_child_node(entry_id, content.clone(), date.clone());
        }
        Commands::Delete { entry_id, date } => {
            client.delete_entry(entry_id, date.clone());
        }
    }
}
