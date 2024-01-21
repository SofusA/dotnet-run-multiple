use clap::Parser;
use rust_search::SearchBuilder;
use std::{env, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ignore: String,
    #[arg(short, long, default_value = "")]
    watch: String,
}

fn main() {
    let args = Args::parse();
    let ignore: String = args.ignore;
    let ignore_list: Vec<String> = ignore.split(' ').map(|s| s.to_string()).collect();

    let path = env::current_dir().unwrap();

    let mut api_list: Vec<String> = SearchBuilder::default()
        .location(path)
        .search_input("")
        .ext("Api")
        .ignore_case()
        .strict()
        .hidden()
        .build()
        .collect();

    ignore_api_list(&mut api_list, &ignore_list);

    let mut watch_path: String = args.watch;
    if watch_path != "" {
        watch_path = find_api_path(&api_list, watch_path);
        ignore_api(&mut api_list, &watch_path);

        Command::new("dotnet")
            .arg("watch")
            .arg("--project")
            .arg(watch_path)
            .spawn()
            .unwrap();
    }

    for path in api_list {
        Command::new("dotnet")
            .arg("run")
            .arg("--project")
            .arg(path)
            .spawn()
            .unwrap();
    }

    loop {}
}

fn ignore_api(api_list: &mut Vec<String>, ignore_api: &str) {
    api_list.retain(|item| !item.contains(ignore_api));
}

fn ignore_api_list(api_list: &mut Vec<String>, ignore_list: &Vec<String>) {
    for ignore_api in ignore_list {
        api_list.retain(|item| !item.contains(ignore_api));
    }
}

fn find_api_path(api_list: &Vec<String>, api: String) -> String {
    let found_path = api_list.into_iter().find(|path| path.contains(&api));

    match found_path {
        Some(path) => return path.clone(),
        None => panic!("Path not found"),
    }
}
