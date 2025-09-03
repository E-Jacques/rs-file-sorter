use crate::core::pipeline::pipeline_data::PipelineContext;

use super::super::error;

use super::{stage::PipelineStage, PipelineData};

pub struct ApplyTransformationStage;
impl PipelineStage<PipelineData, error::Error> for ApplyTransformationStage {
    fn execute(
        &self,
        context: PipelineContext,
        data: PipelineData,
    ) -> Result<PipelineData, error::Error> {
        match data {
            PipelineData::Report(sorter_reports) => {
                let reports = move_files_from_report(sorter_reports);

                if !context.options().root_level_only {
                    remove_empty_directories(
                        &std::path::PathBuf::from(context.input_dir()).as_path(),
                    )
                    .map_err(error::Error::IO)?;
                }

                Ok(PipelineData::Report(reports))
            }
            _ => Err(error::Error::Pipeline),
        }
    }
}

impl std::fmt::Display for ApplyTransformationStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Moving files to target directories...")
    }
}

fn move_files_from_report(
    mut reports: crate::core::report::FullReport,
) -> crate::core::report::FullReport {
    for report in reports.iter_mut() {
        if let Ok(target) = &mut report.result {
            report.result =
                crate::utils::file_manipulator::move_file(&report.input_filename, target, true)
                    .map_err(error::Error::IO)
                    .map_err(std::rc::Rc::new)
                    .map(|_| target.clone());
        }
    }

    reports
}

fn remove_empty_directories(dir: &std::path::Path) -> std::io::Result<()> {
    visit_dirs(
        dir,
        &|_| Ok(()),
        &|entry| match std::fs::remove_dir(entry.path()) {
            Ok(_) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::DirectoryNotEmpty => Ok(()),
            Err(err) => Err(err),
        },
    )
}

fn visit_dirs(
    dir: &std::path::Path,
    file_callback: &dyn Fn(std::fs::DirEntry) -> std::io::Result<()>,
    directory_callback: &dyn Fn(std::fs::DirEntry) -> std::io::Result<()>,
) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, file_callback, directory_callback)?;
                directory_callback(entry)?;
            } else {
                file_callback(entry)?;
            }
        }
    }
    Ok(())
}
