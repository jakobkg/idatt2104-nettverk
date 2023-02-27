use std::{fs::OpenOptions, io::Write, process::Command};

use actix_cors::Cors;
use actix_web::{post, web::Json, App, HttpServer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
enum Language {
    rust,
    c,
    cpp,
}

#[derive(Deserialize)]
struct Program {
    lang: Language,
    program: String,
}

#[derive(Serialize)]
struct CompilerResult {
    compiler_output: String,
    program_output: String,
}

#[post("/")]
async fn compile(json: Json<Program>) -> Json<CompilerResult> {
    let uuid = Uuid::new_v4();
    println!("Mottatt forespørsel: {uuid}");

    let dirname = format!("tmp/{}", uuid.to_string());

    let filename = format!(
        "src.{}",
        match json.lang {
            Language::rust => "rs",
            Language::c => "c",
            Language::cpp => "cpp",
        }
    );

    let fullpath = format!("{}/{}", dirname.clone(), filename.clone());

    let _ = Command::new("mkdir")
        .arg("-p")
        .arg(dirname.clone())
        .output()
        .expect("Kunne ikke opprette tmp-mappe");

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(fullpath.clone())
        .expect("Kunne ikke opprette kildefil");

    file.write(json.program.as_bytes())
        .expect("Kunne ikke skrive program til kildefil");

    let compiler = match json.lang {
        Language::rust => "rustc",
        Language::c => "gcc",
        Language::cpp => "g++",
    };

    let res = Command::new(compiler)
        .arg(fullpath.clone())
        .arg("-o")
        .arg(format!("{dirname}/out"))
        .output()
        .expect("Kunne ikke starte kompilator");

    let compiler_output = format!(
        "{}\n{}\nCompiler finished with {}",
        String::from_utf8_lossy(&res.stdout),
        String::from_utf8_lossy(&res.stderr),
        res.status
    )
    .trim_start()
    .to_string();

    let program_output;

    if res.status.success() {
        let res = Command::new(format!("./{}/out", dirname))
            .output()
            .expect("Programmet kunne ikke starte");
        program_output = String::from_utf8_lossy(&res.stdout).to_string();
    } else {
        program_output = "Did not compile".to_string();
    }

    let _ = Command::new("rm").arg("-r").arg(dirname.clone()).spawn();

    Json(CompilerResult {
        compiler_output,
        program_output,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new().wrap(cors).service(compile)
    })
    .bind(("127.0.0.1", 2000))?
    .run()
    .await
}
