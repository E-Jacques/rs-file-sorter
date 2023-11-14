mod common;

#[cfg(test)]
pub mod tests_e2e_exec_sort_command {
    use std::{fs, path::Path};

    use rs_file_sorter::{
        cli_handler::parser::{ArgValue, ParsedArgs},
        commands::sort_command::exec_sort_command,
    };

    use crate::common::{
        clean_dir, file_creator::FileCreator, file_or_dir_exists, generate_test_files,
    };

    #[test]
    fn test_sort_to_same_file() {
        let files = vec![
            FileCreator::from("file_2022-02-22_F1BDD782"),
            FileCreator::from("file_2022-10-20_6FC02130"),
            FileCreator::from("file_2023-10-20_9E387272"),
        ];
        let target_dir = Path::new("tests")
            .join("rsc")
            .join("sort")
            .join("test_1")
            .join("target_dir");
        clean_dir(target_dir.clone())
            .expect("Should be able to clean directory before running test.");
        generate_test_files(&target_dir, files).expect("Unable to generate the test files!");

        let final_target_dir = target_dir.clone().to_str().unwrap().to_string();
        exec_sort_command(
            vec![ParsedArgs {
                arg_name: String::from("stack"),
                arg_value: ArgValue::Multiple(vec![String::from("year"), String::from("month")]),
            }],
            vec![final_target_dir.clone(), final_target_dir.clone()],
        );

        let content_of_target_dir = fs::read_dir(target_dir.clone()).unwrap();
        assert_eq!(content_of_target_dir.count(), 2);

        // Test if year files have been created.
        let pathbuf_2023_dir = target_dir.clone().join("2023");
        let pathbuf_2022_dir = target_dir.clone().join("2022");
        assert!(file_or_dir_exists(pathbuf_2023_dir.clone()));
        assert!(file_or_dir_exists(pathbuf_2022_dir.clone()));

        let content_2023_dir = fs::read_dir(pathbuf_2023_dir.clone()).unwrap();
        let content_2022_dir = fs::read_dir(pathbuf_2022_dir.clone()).unwrap();
        assert_eq!(content_2023_dir.count(), 1);
        assert_eq!(content_2022_dir.count(), 2);
        assert!(file_or_dir_exists(
            pathbuf_2022_dir
                .clone()
                .join("02_Février")
                .join("file_2022-02-22_F1BDD782")
        ));
        assert!(file_or_dir_exists(
            pathbuf_2022_dir
                .clone()
                .join("10_Octobre")
                .join("file_2022-10-20_6FC02130")
        ));
        assert!(file_or_dir_exists(
            pathbuf_2023_dir
                .clone()
                .join("10_Octobre")
                .join("file_2023-10-20_9E387272")
        ));
    }
}
