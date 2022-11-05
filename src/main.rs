use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

#[macro_use]
extern crate log;

use clap::Parser;

extern crate fs_extra;
use env_logger::Env;
use fs_extra::dir::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of angular frontend dir
    #[arg(short = 'f', long)]
    fepath: String,

    /// [Optional] The path where the built app folder will be deployed to (e.g: nginx/html) and automatically move the built app to the specified directory
    #[arg(short = 'd', long)]
    deploypath: Option<String>,

    /// [Optional] App name to build
    #[arg(short = 'a', long)]
    appname: Option<String>,

    /// Skip nx cache
    #[arg(short = 's', long, action = clap::ArgAction::SetTrue)]
    skipnxcache: Option<bool>,
}

fn change_path(new_path: &str) {
    let my_fe_path = Path::new(new_path);
    assert!(env::set_current_dir(&my_fe_path).is_ok(), "Path not found!");
    info!(
        "Successfully changed working directory to {}",
        my_fe_path.display()
    );
}

fn run_ng_command(my_command: &str) {
    let mut cmd;
    if cfg!(target_os = "windows") {
        cmd = Command::new(format!("cmd"));
        cmd.arg("/C");
    } else {
        cmd = Command::new(format!("sh"));
        cmd.arg("-c");
    };
    cmd.arg(my_command);
    let child = cmd.spawn().expect("Failed executing command!");

    child
        .wait_with_output()
        .expect("Failed waiting child process to finish!");
}

fn execute_delete_dir(server_dir: &str, app_name: &str) {
    info!("Deleting existing app {} in server directory", app_name);
    if Path::new(server_dir).join(app_name).exists() {
        let _delete_result = fs::remove_dir_all(Path::new(server_dir).join(app_name));
    } else {
        info!("No existing app in server directory")
    }

    info!(
        "Done deleting existing app {} in server directory",
        app_name
    )
}

fn execute_move_dir(my_fe_dist_app_path: &PathBuf, server_dir: &str) {
    info!(
        "Moving built app from {} to server directory {}",
        my_fe_dist_app_path.display(),
        server_dir
    );
    let options = CopyOptions {
        ..Default::default()
    };
    let _result_move = fs_extra::dir::move_dir(my_fe_dist_app_path, server_dir, &options);

    info!(
        "Done moving built app from {} to server directory {}",
        my_fe_dist_app_path.display(),
        server_dir
    );
}

fn move_app_dir_to_server_dir(fe_path: &str, app_dir: &str, server_dir: &str, app_name: &str) {
    let my_fe_path = Path::new(fe_path);
    let my_fe_dist_path = &my_fe_path.join(app_dir);
    let my_fe_dist_app_path;
    let mut default_app_name = String::new().to_owned();

    if app_name.is_empty() {
        if cfg!(target_os = "windows") {
            let temp_app_name = fe_path.split('\\');
            default_app_name.push_str(&temp_app_name.last().unwrap().to_string());
        } else {
            let temp_app_name = fe_path.split('/');
            default_app_name.push_str(&temp_app_name.last().unwrap().to_string());
        }
    } else {
        default_app_name.push_str(app_name);
    }

    assert!(
        env::set_current_dir(&my_fe_dist_path).is_ok(),
        "<frontendpath>/dist path not found!"
    );

    info!(
        "Angular {} app build dir found inside {}",
        default_app_name,
        my_fe_dist_path.display()
    );

    my_fe_dist_app_path = my_fe_dist_path.join(&default_app_name);
    execute_delete_dir(server_dir, &default_app_name);
    execute_move_dir(&my_fe_dist_app_path, server_dir);
}

fn main() {
    println!("======================================================");
    println!("= Simple CLI program to build and deploy angular app =");
    println!("= Version: 0.1.0                                     =");
    println!("= Made by Aridya Maharianto, November 2022           =");
    println!("= For personal use only!                             =");
    println!("======================================================");
    println!();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    info!("App started");

    let default_build_command = "npm run ng b";
    let mut command_to_run = String::new().to_owned();
    let mut my_app_name = String::new().to_owned();

    let args = Args::parse();

    if args.appname.is_none() {
        command_to_run.push_str(default_build_command);
    } else {
        my_app_name = args.appname.unwrap();
        command_to_run.push_str(&format!("{} {}", default_build_command, &my_app_name));
    }

    if args.skipnxcache.unwrap_or_default() == true {
        command_to_run.push_str(&format!(" --skip-nx-cache"));
    }

    info!("Running command: {}", command_to_run);

    change_path(&args.fepath);
    run_ng_command(&command_to_run);

    if let Some(deploypath) = args.deploypath.as_deref() {
        move_app_dir_to_server_dir(&args.fepath, "dist", deploypath, &my_app_name);
    } else {
        info!("Automatic deployment did not run, deploypath flag not specified!");
        info!("use angular-deploy -h or angular-deploy --help for usage info!");
    }
    info!("Graceful shutdown!")
}
