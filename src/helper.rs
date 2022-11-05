use std::{env, path::Path};

use itertools::Itertools;

pub fn get_app_names(app_names: &[String]) -> Vec<&str> {
    let mut retval: Vec<&str> = vec![];
    for ii in 0..app_names.len() {
        retval.push(app_names[ii].as_str())
    }

    retval = retval.into_iter().unique().collect();

    return retval;
}

pub fn change_path(new_path: &str) {
    let my_fe_path = Path::new(new_path);
    assert!(env::set_current_dir(&my_fe_path).is_ok(), "Path not found!");
    info!(
        "Successfully changed working directory to {}",
        my_fe_path.display()
    );
}
