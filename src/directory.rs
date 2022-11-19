extern crate fs_extra;
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use fs_extra::dir::*;

fn execute_delete_dir(server_dir: &str, app_name: &str) -> Result<(), Box<dyn Error>> {
    info!("Deleting existing app {} in server directory", app_name);
    if Path::new(server_dir).join(app_name).exists() {
        let _delete_result = fs::remove_dir_all(Path::new(server_dir).join(app_name))?;
    } else {
        warn!("No existing app {} in server directory", app_name);
    }

    info!(
        "Done deleting existing app {} in server directory",
        app_name
    );
    Ok(())
}

fn execute_move_dir(my_fe_dist_app_path: &PathBuf, server_dir: &str) -> Result<(), Box<dyn Error>> {
    info!(
        "Moving built app from {} to server directory {}",
        my_fe_dist_app_path.display(),
        server_dir
    );

    if Path::new(my_fe_dist_app_path).exists() {
        let options = CopyOptions {
            ..Default::default()
        };
        let _result_move = fs_extra::dir::move_dir(my_fe_dist_app_path, server_dir, &options)?;
    } else {
        warn!(
            "{} not found! Please check your angular build log for more details",
            my_fe_dist_app_path.display()
        );
    }

    info!(
        "Done moving built app from {} to server directory {}",
        my_fe_dist_app_path.display(),
        server_dir
    );
    Ok(())
}

pub fn move_app_dir_to_server_dir(
    fe_path: &str,
    app_dir: &str,
    server_dir: &str,
    app_name: &str,
) -> Result<(), Box<dyn Error>> {
    let my_fe_path = Path::new(fe_path);
    let my_fe_dist_path = &my_fe_path.join(app_dir);
    let my_fe_dist_nx_path = &my_fe_dist_path.join("apps");
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

    match my_fe_dist_nx_path.is_dir() {
        true => {
            env::set_current_dir(&my_fe_dist_nx_path)?;
            info!("Angular build dir found {}", my_fe_dist_nx_path.display());
            my_fe_dist_app_path = my_fe_dist_nx_path.join(&default_app_name);
            execute_delete_dir(server_dir, &default_app_name)?;
            execute_move_dir(&my_fe_dist_app_path, server_dir)?;
            Ok(())
        }
        false => {
            error!("<frontendpath>/dist path not found or is not directory");
            return Err("Moving app(s) dir to server dir failed".into());
        }
    }
}
