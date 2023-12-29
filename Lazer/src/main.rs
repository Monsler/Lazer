use std::{env, fs, path::Path, fs::File, io::Write};
use reqwest::blocking::get;
//use json::{self, JsonValue};
use base64::{Engine as _, engine::general_purpose};
use serde_json::{json, Value};

// Lazer made by Monsler



fn main() {
    let VERSION_EXPORT = "1-1-0";
    let args: Vec<String> = env::args().collect();

    let str = "| Lazer | Lazurite package helper |";
    let mut longer = String::new();
    for _ in 1..=str.len() {
        longer.push('=');
    }
    println!("{}", longer);
    println!("{}", str);
    println!("{}", longer);



    if 1 < args.len() {
        let arg = &args[1];
        if arg == "new" {
            let name = &args[2];
            let pth = &args[3];
            fs::create_dir(&format!("{}/{}", pth, name)).unwrap();
            fs::create_dir(&format!("{}/{}/src", pth, name)).unwrap();
            fs::create_dir(&format!("{}/{}/src/lib", pth, name)).unwrap();
            let mut index = File::create(&format!("{pth}/{name}/src/index.lzr")).expect("ERROR");
            let mut mark = File::create(&format!("{pth}/{name}/.lazer")).expect("ERROR");
            mark.write_all("{}".as_bytes()).expect("ERROR");
            index.write_all("println(\"Hello world\")".as_bytes()).expect("ERROR");

            println!("Project <{name}> created successfully at <{pth}>.")
        }else if arg == "install" {
            let libname = &args[2];
            let path = &args[3];
            println!("Looking for library <{libname}>...\n");
            let response = get(&format!("https://lazer-repo-default-rtdb.firebaseio.com/{libname}.json")).unwrap();
            let body = response.text().unwrap();
            let resp = json::parse(&body).unwrap();
            if resp.len() > 0 && Path::new(&format!("{path}/.lazer")).exists() && Path::new(path).exists() {
                let json_value = resp["code"].to_string();
                let code = general_purpose::STANDARD.decode(json_value).unwrap();
                println!("Library <{libname}> downloaded successfully. Author: {}; Version: {}", resp["author"], resp["version-pkg"]);
                let mut lib = File::create(&format!("{path}/src/lib/{libname}.lzr")).expect("ERROR");
                lib.write_all(&code).expect("ERROR");
                println!("Saved as <{path}/src/lib/{libname}.lzr>.\n");
                let lazer_t = fs::read_to_string(&format!("{path}/.lazer")).unwrap();
                let mut lazer_vec = json::parse(&lazer_t).unwrap();
                let _ = lazer_vec[&format!("{libname}")] = resp["version-pkg"].clone();
                let mut mark = File::create(&format!("{path}/.lazer")).unwrap();
                mark.write_all(json::stringify(lazer_vec).as_bytes()).expect("ERROR");
            }else{
                println!("Library <{libname}> isn't found on server or .lazer file not found. Try to change your input!\n\n");
            }
        }else if arg == "remove" {
            let path = &args[2];
            if Path::new(&path).exists() && Path::new(&format!("{path}/.lazer")).exists() {
                let _ = fs::remove_dir_all(path);
                println!("<{path}> deleted successfully.\n");
            }else{
                println!("Warning: unsafe delete of <{path}>.\nLazer mark not found!\n");
            }
        }else if arg == "unset" {
            let libname = &args[2];
            let path = &args[3];

            if Path::new(path).exists() && Path::new(&format!("{path}/src/lib/{libname}.lzr")).exists() {
                let _ = fs::remove_file(&format!("{path}/src/lib/{libname}.lzr"));
                println!("Library <{libname}> removed successfully.\n");
            }
        }else if arg == "available-libs" {
            let response = get("https://lazer-repo-default-rtdb.firebaseio.com/.json").unwrap();
            let body = response.text().unwrap();
            let resp: Vec<json::JsonValue> = json::parse(&body).into_iter().collect();
            let mut int = 0;
            println!("{body}");
        }else if arg == "version-running" {
            println!("Version: {VERSION_EXPORT}\n");
        }else if arg == "update" {
            let libname = &args[2];
            let path = &args[3];
            println!("Looking for library <{libname}> updates...\n");
            let response = get(&format!("https://lazer-repo-default-rtdb.firebaseio.com/{libname}.json")).unwrap();
            let body = response.text().unwrap();
            let resp = json::parse(&body).unwrap();
            if resp.len() > 0 && Path::new(&format!("{path}/.lazer")).exists() && Path::new(path).exists() {
                let lazer_t = fs::read_to_string(&format!("{path}/.lazer")).unwrap();
                let lazer_vec = json::parse(&lazer_t).unwrap();
                let cuver = lazer_vec[&format!("{libname}")].to_string();
                let newver = resp["version-pkg"].to_string();
                if cuver != newver {
                    println!("Update found. Library <{libname}> is downloading (v. {newver}) \n");
                    let json_value = resp["code"].to_string();
                    let code = general_purpose::STANDARD.decode(json_value).unwrap();
                    let mut lib = File::create(&format!("{path}/src/lib/{libname}.lzr")).expect("ERROR");
                    lib.write_all(&code).expect("ERROR");
                    println!("Saved <{libname}> as <{path}/src/lib/{libname}.lzr>; New version: {newver}.\n");
                    let lazer_t = fs::read_to_string(&format!("{path}/.lazer")).unwrap();
                    let mut lazer_vec = json::parse(&lazer_t).unwrap();
                    let _ = lazer_vec[&format!("{libname}")] = resp["version-pkg"].clone();
                    let mut mark = File::create(&format!("{path}/.lazer")).unwrap();
                    mark.write_all(json::stringify(lazer_vec).as_bytes()).expect("ERROR");
                }else if newver == cuver {
                    println!("You are using the last version of <{libname}>, chill :)");
                }
            }
        }
    }else{
        println!("It's looks like you need a help.\n\nMethods:\nnew - create new project. Syntax: new [name,] [directory]\nrun - Runs the project. Syntax: run [directory]\nremove - Destroys the project. Syntax: remove [directory]\ninstall - Installs library into your project. Syntax: install [libname,] [directory_project]\navailable-libs - Scan the repo for libraries name. Syntax: available-libs\nunset - Removes library from project. Syntax: unset [libname,] [directory]\nupdate - Check for library's update. Syntax: update [libname,] [directory]\n:3\nIf you want to upload your own library, dm me on discord: @monsler\n\n");
    }
}
