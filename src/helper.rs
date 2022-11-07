use std::{env, path::Path};

use itertools::Itertools;

#[derive(Clone)]
pub struct AppNamesStruct<'a> {
    pub core_app_names: Vec<&'a str>,
    pub portal_app_names: Vec<&'a str>,
}

pub fn get_app_names(app_names: &[String]) -> AppNamesStruct {
    let mut retval = AppNamesStruct {
        core_app_names: vec![],
        portal_app_names: vec![],
    };
    for ii in 0..app_names.len() {
        if app_names[ii].as_str().ends_with("-portal") {
            retval.portal_app_names.push(app_names[ii].as_str())
        } else {
            retval.core_app_names.push(app_names[ii].as_str())
        }
    }

    retval.core_app_names = retval.core_app_names.into_iter().unique().collect();
    retval.portal_app_names = retval.portal_app_names.into_iter().unique().collect();

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
