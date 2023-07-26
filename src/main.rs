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
use octocrab::models::repos::{CommitAuthor, Object};
use octocrab::Octocrab;
use octocrab::params::repos::Reference;
use time::macros::{format_description, offset};
use time::{format_description, OffsetDateTime, UtcOffset};
use time::format_description::well_known;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // // initialize tracing
    // tracing_subscriber::fmt::init();

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
        .route("/", post(|x| create_comment(x, crab)));

    let port = match std::env::var("JKC_PORT") {
        Ok(p) => {
            match u16::from_str(p.as_str()) {
                Ok(port) => port,
                Err(_) => {
                    eprintln!("Cannot convert value of $JKC_PORT ({p}) to u16");
                    exit(1);
                }
            }
        }
        Err(_) => {
            println!("No value for $JKC_PORT; using 10113");
            10113u16;
        }
    };

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

async fn create_comment(Form(payload): Form<Comment>, crab: Octocrab) -> Redirect {
    println!("received comment {:?}", payload);
    let uuid = Uuid::new_v4();
    let post_id = payload.post_id;
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
name: {}
email: {}
gravatar: {:x}
", &payload.name, &payload.email, md5::compute(&payload.email));
    if let Some(url) = payload.website {
        file_contents.push_str(format!("url: {url}\n").as_str());
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
    Redirect::to(payload.redirect.as_str())
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