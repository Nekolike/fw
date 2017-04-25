use config;
use config::Project;
use errors::AppError;
use slog::Logger;


pub fn ls(maybe_config: Result<config::Config, AppError>) -> Result<(), AppError> {
  let config = maybe_config?;
  let output = config.projects
                     .into_iter()
                     .map(|(_, p)| println!("{}", p.name));
  for _ in output {}
  Ok(())
}

pub fn gen(name: &str, maybe_config: Result<config::Config, AppError>, quick: bool, logger: &Logger) -> Result<(), AppError> {
  let config = maybe_config?;
  let project: &Project = config.projects
                                .get(name)
                                .ok_or_else(|| AppError::UserError(format!("project key {} not found in ~/.fw.json", name)))?;
  let canonical_project_path = config::actual_path_to_project(&config.settings.workspace, project);
  let path = canonical_project_path.to_str()
                                   .ok_or(AppError::InternalError("project path is not valid unicode"))?;
  if !canonical_project_path.exists() {
    Err(AppError::UserError(format!("project key {} found but path {} does not exist",
                                    name,
                                    path)))
  } else {
    let after_workon = if !quick {
      project.after_workon
             .clone()
             .map(|cmd| prepare_workon(cmd))
             .or_else(|| config.resolve_workon_from_tags(project.tags.clone(), logger).map(prepare_workon))
             .unwrap_or_else(|| "".to_owned())
    } else {
      String::new()
    };
    println!("cd {}{}", path, after_workon);
    Ok(())
  }
}

fn prepare_workon(workon: String) -> String {
  format!(" && {}", workon)
}
