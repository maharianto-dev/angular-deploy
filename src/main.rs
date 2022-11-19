use std::error::Error;

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
    /// Full path of angular frontend dir
    #[arg(short = 'f', long)]
    fepath: String,

    /// [Optional] The full path where the built app folder will be deployed to (e.g: nginx/html) and automatically move the built app to the specified directory
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

#[derive(PartialEq)]
enum AppKind {
    Core,
    Portal,
}

fn main() {
    println!("======================================================");
    println!("= Simple CLI program to build and deploy angular app =");
    println!("= Version: 0.1.3                                     =");
    println!("= Created by Aridya Maharianto, November 2022        =");
    println!("= For personal use only!                             =");
    println!("======================================================");
    println!();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    info!("App started");

    let command_to_run = String::new().to_owned();

    let args = Args::parse();

    let app_names = helper::get_app_names(&args.appname);

    if app_names.core_app_names.len() == 0 && app_names.portal_app_names.len() == 0 {
        error!("Please provide app name(s) using -a or --appname flag");
        error!("Run angular-deploy -h or angular-deploy --help for usage");
        error!("Exiting application...");
        panic!();
    }

    let is_nx = args.nx.unwrap_or_default();
    let is_skip_nx_cache = args.skipnxcache.unwrap_or_default();

    if app_names.core_app_names.len() == 0 {
        warn!("Detected 0 (zero) core app in supplied argument, skipping core apps build and deployment");
    } else {
        info!("Starting core app(s) build and deployment");
        build_and_deploy(
            app_names.core_app_names,
            &command_to_run,
            is_nx,
            is_skip_nx_cache,
            &args.fepath,
            args.deploypath.clone(),
            AppKind::Core,
        )
        .expect("Failed building and deploying core app(s)");
        info!("Done with core app(s) build and deployment");
    }

    if app_names.portal_app_names.len() == 0 {
        warn!("Detected 0 (zero) portal app in supplied argument, skipping portal apps build and deployment")
    } else {
        info!("Starting portal app(s) build and deployment");
        build_and_deploy(
            app_names.portal_app_names,
            &command_to_run,
            is_nx,
            is_skip_nx_cache,
            &args.fepath,
            args.deploypath.clone(),
            AppKind::Portal,
        )
        .expect("Failed building and deploying portal app(s)");
        info!("Done with portal app(s) build and deployment");
    }

    info!("Graceful shutdown!")
}

fn build_and_deploy(
    app_names: Vec<&str>,
    command_to_run_p: &str,
    is_nx: bool,
    is_skip_nx_cache: bool,
    fe_path: &str,
    deploy_path: Option<String>,
    kind: AppKind,
) -> Result<(), Box<dyn Error>> {
    let mut command_to_run = command_to_run_p.to_string();
    if app_names.len() > 1 || is_nx == true {
        if app_names.len() > 1 {
            command_to_run.push_str(&format!(
                "nx run-many --target=build --projects={} --parallel={}",
                app_names.join(","),
                app_names.len()
            ));
        } else {
            command_to_run.push_str(&format!("nx b {}", app_names[0]));
        }
    } else {
        command_to_run.push_str(&format!("ng b {}", app_names[0]));
    }

    if is_skip_nx_cache == true {
        command_to_run.push_str(&format!(" --skip-nx-cache"));
    }

    if kind == AppKind::Portal {
        command_to_run.push_str(&format!(" --configuration=portal"));
    }

    info!("Running command: {}", command_to_run);

    helper::change_path(fe_path).expect("Error changing path to supplied frontend path");
    angular::run_ng_command(&command_to_run).expect("Error running angular command");

    if let Some(deploypath) = deploy_path.as_deref() {
        if app_names.len() > 1 {
            for ii in 0..app_names.len() {
                directory::move_app_dir_to_server_dir(fe_path, "dist", deploypath, app_names[ii])
                    .expect(
                        format!("Failed moving app dir to server dir for {}", app_names[ii])
                            .as_str(),
                    );
            }
        } else {
            directory::move_app_dir_to_server_dir(fe_path, "dist", deploypath, app_names[0])
                .expect(
                    format!("Failed moving app dir to server dir for {}", app_names[0]).as_str(),
                );
        }
    } else {
        info!("Automatic deployment did not run, deploypath flag not specified!");
        info!("use angular-deploy -h or angular-deploy --help for usage info!");
    }
    Ok(())
}
