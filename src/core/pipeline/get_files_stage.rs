use crate::core::pipeline::pipeline_data::PipelineContext;

use super::super::error;

use super::{stage::PipelineStage, PipelineData};

pub struct GetFilesStage;
impl PipelineStage<PipelineData, error::Error> for GetFilesStage {
    fn execute(
        &self,
        context: PipelineContext,
        data: PipelineData,
    ) -> Result<PipelineData, error::Error> {
        match data {
            PipelineData::Empty => {
                let files_list: Result<Vec<std::path::PathBuf>, error::Error> =
                    if context.options().root_level_only {
                        std::fs::read_dir(&context.input_dir())
                            .map_err(error::Error::IO)?
                            .filter_map(|entry| match entry {
                                Ok(e) => match e.file_type() {
                                    Ok(file_type) if file_type.is_file() => Some(Ok(e)),
                                    _ => None,
                                },
                                Err(err) => Some(Err(err)),
                            })
                            .map(|entry| entry.map(|e| e.path()))
                            .collect::<Result<_, _>>()
                            .map_err(error::Error::IO)
                    } else {
                        read_recursively(std::path::Path::new(&context.input_dir()))
                            .map_err(error::Error::IO)
                    };

                Ok(PipelineData::Paths(files_list?))
            }
            _ => Err(error::Error::Pipeline),
        }
    }
}

impl std::fmt::Display for GetFilesStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Retrieving files...")
    }
}

fn read_recursively(dir: &std::path::Path) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut files = vec![];

    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recursive call
                let nested_files = read_recursively(&path)?;
                files.extend(nested_files);
            } else {
                files.push(entry.path());
            }
        }
    }

    Ok(files)
}
