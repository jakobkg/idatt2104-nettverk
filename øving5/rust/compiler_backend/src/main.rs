use std::{fs::OpenOptions, io::Write, process::Command};

use actix_cors::Cors;
use actix_web::{error::InternalError, http::StatusCode, post, web::Json, App, HttpServer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
enum Language {
    rust,
    c,
    cpp,
}

impl Language {
    fn extension(&self) -> String {
        format!(
            "{}",
            match self {
                Language::rust => "rs",
                Language::c => "c",
                Language::cpp => "cpp",
            }
        )
    }

    fn compiler(&self) -> String {
        format!(
            "{}",
            match self {
                Language::rust => "rustc",
                Language::c => "gcc",
                Language::cpp => "g++",
            }
        )
    }
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
async fn compile(request: Json<Program>) -> Result<Json<CompilerResult>, actix_web::Error> {
    let uuid = Uuid::new_v4();
    println!("Mottatt forespørsel: {} av type {:?}", uuid, request.lang);

    let dirname = format!("tmp/{}", uuid.to_string());
    let filename = format!("src.{}", request.lang.extension());
    let fullpath = format!("{}/{}", dirname.clone(), filename.clone());

    // Forsøk å opprette en mappe med navn fra UUID, svar med HTTP 500 om dette feiler
    if let Err(_) = Command::new("mkdir")
        .arg("-p")
        .arg(dirname.clone())
        .output()
    {
        return Err(InternalError::new(
            "Kunne ikke opprette midlertidig mappe for oppbevaring av kode",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into());
    }

    // Forsøk å opprette fil for å midlertidig lagre kildekode, svar med HTTP 500 om dette feiler
    let mut file = match OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(fullpath.clone())
    {
        Ok(file) => file,
        Err(_) => {
            return Err(InternalError::new(
                "Kunne ikke opprette midlertidig fil for kildekode",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into())
        }
    };

    // Forsøk å skrive programkode til den midlertidige filen, returner HTTP 500 om dette feiler
    if let Err(_) = file.write(request.program.as_bytes()) {
        return Err(InternalError::new(
            "Kunne ikke skrive kildekode til midlertidig fil",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into());
    }

    // Forsøk å kompilere programkoden, returner vanlig resultat om kompileringen kan starte men feiler pga kodefeil
    // Returnerer HHTP 500 om kompilator-prosessen ikke kunne startes
    let res = match Command::new(request.lang.compiler())
        .arg(fullpath.clone())
        .arg("-o")
        .arg(format!("{dirname}/out"))
        .output()
    {
        Ok(output) => output,
        Err(_) => {
            return Err(InternalError::new(
                "Kunne ikke starte kompilator-prosess",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into())
        }
    };

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
        // Prøv å kjøre det kompilerte programmet, svar med HTTP 500 om oppstart av prosessen feiler
        if let Err(_) = Command::new(format!("./{}/out", dirname)).output() {
            return Err(InternalError::new(
                "Kunne ikke kjøre kompilert program",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into());
        }

        program_output = format!(
            "{}\nProgram finished with {}",
            String::from_utf8_lossy(&res.stdout),
            res.status
        )
        .trim_start()
        .to_string();
    } else {
        program_output = "Did not compile".to_string();
    }

    let _ = Command::new("rm").arg("-r").arg(dirname.clone()).spawn();

    Ok(Json(CompilerResult {
        compiler_output,
        program_output,
    }))
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
