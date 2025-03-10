use std::env::VarError;
use std::fmt::format;
use axum::{routing::{get, post}, http::StatusCode, response::IntoResponse, Json, Router, Form};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::num::ParseIntError;
use std::process::exit;
use std::str::FromStr;
use axum::handler::Handler;
use axum::response::Redirect;
use clap::{Arg, Parser};
use octocrab::models::repos::{CommitAuthor, Object};
use octocrab::Octocrab;
use octocrab::params::repos::Reference;
use regex::Regex;
use time::macros::{format_description, offset};
use time::{format_description, OffsetDateTime, UtcOffset};
use time::format_description::well_known;
use uuid::Uuid;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 10113)]
    port: u16
}

#[tokio::main]
async fn main() {
    // // initialize tracing
    // tracing_subscriber::fmt::init();

    let port = Args::parse().port;

    let token = match std::env::var("GITHUB_TOKEN") {
        Ok(token_string) => token_string,
        Err(_) => {
            eprintln!("GITHUB_TOKEN must be set and be valid Unicode");
            exit(1);
        }
    };

    let crab = match octocrab::Octocrab::builder()
        .personal_token(token)
        .build() {
        Ok(crab) => crab,
        Err(_) => {
            eprintln!("Unable to connect to GitHub");
            exit(1);
        }
    };

    // build our application with a route
    let app = Router::new()
        .route("/comment", post(|x| create_comment(x, crab)))
        .route("/status", get(return_ok))
    ;

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    // tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn return_ok() -> &'static str {
    "The comments service is functioning normally\n"
}

async fn create_comment(Form(payload): Form<Comment>, crab: Octocrab) -> Redirect {
    println!("received comment {:?}", payload);
    let uuid = Uuid::new_v4();
    let invalid_post_id_chars = Regex::new(r"[^a-zA-Z0-9-]").unwrap();
    let post_id = invalid_post_id_chars.replace_all(payload.post_id.as_str(), "");
    // At the moment I am not checking to see if the passed post_id is a valid post, just whether it is safe.
    println!("uuid: {uuid}");
    println!("post: {post_id}");
    let branch_name = format!("comment/{post_id}/{uuid}");
    let repos = crab.repos("Samasaur1", "samasaur1.github.io");
    let sha = match repos.get_ref(&Reference::Branch("main".to_string())).await.unwrap().object {
        Object::Commit { sha, url} => sha,
        Object::Tag { sha, url } => sha,
        _ => { panic!() }
    };
    println!("got main ref");
    repos.create_ref(&Reference::Branch(branch_name.clone()), sha).await.unwrap();
    let mut file_contents = format!("\
id: {uuid}
name: |-
  {}
email: |-
  {}
gravatar: {:x}
", &payload.name.replace("\n", " "), &payload.email.replace("\n", " "), md5::compute(&payload.email));
    if let Some(url) = payload.website {
        file_contents.push_str(format!("url: |-\n  {}\n", url.replace("\n", " ")).as_str());
    }
    let now = OffsetDateTime::now_utc();
    file_contents.push_str(format!("date: {}\n", now.format(&well_known::Iso8601::DEFAULT).unwrap()).as_str());
    file_contents.push_str("message: |-2\n");
    let lines = payload.message.split("\n");
    for line in lines {
        file_contents.push_str(format!("  {line}\n").as_str());
    }
    let file_update = repos
        .create_file(
            format!("_data/comments/{}/{uuid}.yml", post_id),
            format!("Add comment on {}", post_id),
            file_contents)
        .branch(&branch_name)
        .author(CommitAuthor {
            //TODO: asciify?
            name: payload.name,
            email: payload.email,
        })
        .commiter(CommitAuthor {
            name: "Samasaur".to_string(),
            email: "73031317+Samasaur@users.noreply.github.com".to_string(),
        })
        .send()
        .await
        .unwrap();
    println!("created file on branch");
    crab.pulls("Samasaur1", "samasaur1.github.io")
        .create(format!("Add comment on {}", post_id), branch_name, "main")
        .send().await.unwrap();
    let asciify = Regex::new(r"[^[:ascii:]]").unwrap();
    Redirect::to(&*asciify.replace_all(payload.redirect.as_str(), ""))
}

// the input to our `create_user` handler
#[derive(Deserialize, Debug)]
struct Comment {
    name: String,
    email: String,
    website: Option<String>,
    message: String,
    post_id: String,
    redirect: String
}