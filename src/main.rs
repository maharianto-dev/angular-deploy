use clap::Parser;
use env_logger::Env;

#[macro_use]
extern crate log;

mod angular;
mod directory;
mod helper;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of angular frontend dir
    #[arg(short = 'f', long)]
    fepath: String,

    /// [Optional] The path where the built app folder will be deployed to (e.g: nginx/html) and automatically move the built app to the specified directory
    #[arg(short = 'd', long)]
    deploypath: Option<String>,

    /// List of app name(s) to build delimited with comma (,), will automatically use nx instead of ng when provided with more than 1 app names
    #[arg(short = 'a', long, value_delimiter = ',', action = clap::ArgAction::Set)]
    appname: Vec<String>,

    /// Use nx instead of ng
    #[arg(long, action = clap::ArgAction::SetTrue)]
    nx: Option<bool>,

    /// Skip nx cache
    #[arg(short = 's', long, action = clap::ArgAction::SetTrue)]
    skipnxcache: Option<bool>,
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

    let default_build_command = "npm run";
    let mut command_to_run = String::new().to_owned();

    let args = Args::parse();

    let app_names = helper::get_app_names(&args.appname);

    if app_names.len() == 0 {
        error!("Please provide app name(s) using -a or --appname flag");
        error!("Run angular-deploy -h or angular-deploy --help for usage");
        error!("Exiting application...");
        panic!();
    }

    if app_names.len() > 1 || args.nx.unwrap_or_default() == true {
        if app_names.len() > 1 {
            command_to_run.push_str(&format!(
                "nx run-many --target=build --projects={} --parallel={}",
                app_names.join(","),
                app_names.len() - 1
            ));
        } else {
            command_to_run.push_str(&format!("nx b {}", app_names[0]));
        }
    } else {
        command_to_run.push_str(&format!("{} ng b {}", default_build_command, app_names[0]));
    }

    if args.skipnxcache.unwrap_or_default() == true {
        command_to_run.push_str(&format!(" --skip-nx-cache"));
    }

    info!("Running command: {}", command_to_run);

    helper::change_path(&args.fepath);
    angular::run_ng_command(&command_to_run);

    if let Some(deploypath) = args.deploypath.as_deref() {
        if app_names.len() > 1 {
            for ii in 0..app_names.len() {
                directory::move_app_dir_to_server_dir(
                    &args.fepath,
                    "dist",
                    deploypath,
                    app_names[ii],
                );
            }
        } else {
            directory::move_app_dir_to_server_dir(&args.fepath, "dist", deploypath, app_names[0]);
        }
    } else {
        info!("Automatic deployment did not run, deploypath flag not specified!");
        info!("use angular-deploy -h or angular-deploy --help for usage info!");
    }
    info!("Graceful shutdown!")
}
