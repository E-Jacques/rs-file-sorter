#[cfg(test)]
pub mod tests_e2e_sort_command {
    use std::{fs, path::Path};

    use rs_file_sorter::cli::handle;
    use rsft_utils::{
        common::{clean_or_create_dir, file_or_dir_exists, generate_test_files},
        file_creator::FileCreator,
    };

    #[test]
    fn test_sort_to_same_directory() {
        // set filenames
        let files = vec![
            FileCreator::from("file_2022-02-22_F1BDD782"),
            FileCreator::from("file_2022-10-20_6FC02130"),
            FileCreator::from("file_2023-10-20_9E387272"),
        ];

        // set target directory & generate test files
        let target_dir = Path::new("tests")
            .join("rsc")
            .join("sort")
            .join("test_1")
            .join("target_dir");
        clean_or_create_dir(target_dir.clone())
            .expect("Should be able to clean or create directory before running test");
        generate_test_files(&target_dir, files).expect("Unable to generate the test files!");

        let final_target_dir = target_dir.clone().to_str().unwrap().to_string();
        handle(
            format!(
                "sort --stack year --stack month {} {}",
                final_target_dir.clone(),
                final_target_dir.clone()
            ),
            Some(true),
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
                .join("02_February")
                .join("file_2022-02-22_F1BDD782")
        ));
        assert!(file_or_dir_exists(
            pathbuf_2022_dir
                .clone()
                .join("10_October")
                .join("file_2022-10-20_6FC02130")
        ));
        assert!(file_or_dir_exists(
            pathbuf_2023_dir
                .clone()
                .join("10_October")
                .join("file_2023-10-20_9E387272")
        ));
    }

    #[test]
    fn test_sort_to_different_directories() {
        // set filenames
        let files = vec![
            FileCreator::from("file_2022-02-22_F1BDD782"),
            FileCreator::from("file_2022-10-20_6FC02130"),
            FileCreator::from("file_2023-10-20_9E387272"),
        ];

        let common_dir = &Path::new("tests").join("rsc").join("sort").join("test_2");

        // define input_dir
        let input_dir = common_dir.clone().join("input_dir");
        clean_or_create_dir(input_dir.clone())
            .expect("Should be able to clean or create directory before running test");

        // define output dir
        let output_dir = common_dir.clone().join("output_dir");
        clean_or_create_dir(output_dir.clone())
            .expect("Should be able to clean or create directory before running test");

        // generate files in input directory
        generate_test_files(&input_dir, files).expect("Unable to generate the test files!");

        let final_input_dir = input_dir.clone().to_str().unwrap().to_string();
        let final_output_dir = output_dir.clone().to_str().unwrap().to_string();
        handle(
            format!(
                "sort --stack year --stack month {} {}",
                final_input_dir.clone(),
                final_output_dir.clone()
            ),
            Some(true),
        );

        let content_of_target_dir = fs::read_dir(output_dir.clone()).unwrap();
        assert_eq!(content_of_target_dir.count(), 2);

        // Test if year files have been created.
        let pathbuf_2023_dir = output_dir.clone().join("2023");
        let pathbuf_2022_dir = output_dir.clone().join("2022");
        assert!(file_or_dir_exists(pathbuf_2023_dir.clone()));
        assert!(file_or_dir_exists(pathbuf_2022_dir.clone()));

        let content_2023_dir = fs::read_dir(pathbuf_2023_dir.clone()).unwrap();
        let content_2022_dir = fs::read_dir(pathbuf_2022_dir.clone()).unwrap();
        assert_eq!(content_2023_dir.count(), 1);
        assert_eq!(content_2022_dir.count(), 2);
        assert!(file_or_dir_exists(
            pathbuf_2022_dir
                .clone()
                .join("02_February")
                .join("file_2022-02-22_F1BDD782")
        ));
        assert!(file_or_dir_exists(
            pathbuf_2022_dir
                .clone()
                .join("10_October")
                .join("file_2022-10-20_6FC02130")
        ));
        assert!(file_or_dir_exists(
            pathbuf_2023_dir
                .clone()
                .join("10_October")
                .join("file_2023-10-20_9E387272")
        ));
    }
    #[test]
    fn test_sort_with_parameters_and_concat() {
        // set filenames
        let files = vec![
            FileCreator::from("file_2022-02-22_F1BDD782"),
            FileCreator::from("file_2022-10-20_6FC02130"),
            FileCreator::from("file_2022-10-04_1FCF21G8"),
            FileCreator::from("file_2023-10-20_9E387272"),
        ];

        let common_dir = &Path::new("tests").join("rsc").join("sort").join("test_6");

        // define input_dir
        let input_dir = common_dir.clone().join("input_dir");
        clean_or_create_dir(input_dir.clone())
            .expect("Should be able to clean or create directory before running test");

        // define output dir
        let output_dir = common_dir.clone().join("output_dir");
        clean_or_create_dir(output_dir.clone())
            .expect("Should be able to clean or create directory before running test");

        // generate files in input directory
        generate_test_files(&input_dir, files).expect("Unable to generate the test files!");

        let final_input_dir = input_dir.clone().to_str().unwrap().to_string();
        let final_output_dir = output_dir.clone().to_str().unwrap().to_string();
        handle(
            format!(
                "sort --stack text --parameter value=base_directory --stack concat --parameter strategies=month --parameter strategies=year {} {}",
                final_input_dir.clone(),
                final_output_dir.clone()
            ),
            Some(true),
        );

        let content_of_target_dir = fs::read_dir(output_dir.clone()).unwrap();
        assert_eq!(content_of_target_dir.count(), 1);

        // Test if year files have been created.
        let pathbuf_base_dir = output_dir.clone().join("base_directory");
        assert!(file_or_dir_exists(pathbuf_base_dir.clone()));

        let content_base_dir = fs::read_dir(pathbuf_base_dir.clone()).unwrap();
        assert_eq!(content_base_dir.count(), 3);

        assert!(file_or_dir_exists(
            pathbuf_base_dir
                .clone()
                .join("02_February2022")
                .join("file_2022-02-22_F1BDD782")
        ));
        assert!(file_or_dir_exists(
            pathbuf_base_dir
                .clone()
                .join("10_October2022")
                .join("file_2022-10-20_6FC02130")
        ));
        assert!(file_or_dir_exists(
            pathbuf_base_dir
                .clone()
                .join("10_October2022")
                .join("file_2022-10-04_1FCF21G8")
        ));
        assert!(file_or_dir_exists(
            pathbuf_base_dir
                .clone()
                .join("10_October2023")
                .join("file_2023-10-20_9E387272")
        ));
    }

    #[test]
    #[should_panic = "[ERROR] [Sort Command] input directory 'tests/rsc/sort/test_unknown/input_dir' don't exists"]
    fn test_input_directory_dont_exists() {
        // define input & output directory
        let common_dir = &Path::new("tests")
            .join("rsc")
            .join("sort")
            .join("test_unknown");
        let input_dir = common_dir.clone().join("input_dir");
        let output_dir = common_dir.clone().join("output_dir");

        let final_input_dir = input_dir.clone().to_str().unwrap().to_string();
        let final_output_dir = output_dir.clone().to_str().unwrap().to_string();
        handle(
            format!(
                "sort --stack year --stack month {} {}",
                final_input_dir.clone(),
                final_output_dir.clone()
            ),
            Some(true),
        );
    }

    #[test]
    #[should_panic = "[ERROR] [Sort Command] output directory 'tests/rsc/sort/test_3/output_dir' don't exists"]
    fn test_output_directory_dont_exists() {
        // set filenames
        let files = vec![
            FileCreator::from("file_2022-02-22_F1BDD782"),
            FileCreator::from("file_2022-10-20_6FC02130"),
            FileCreator::from("file_2023-10-20_9E387272"),
        ];

        // define input & output directory
        let common_dir = &Path::new("tests").join("rsc").join("sort").join("test_3");
        let input_dir = common_dir.clone().join("input_dir");
        let output_dir = common_dir.clone().join("output_dir");

        // clean up input directory
        clean_or_create_dir(input_dir.clone())
            .expect("Should be able to clean or create directory before running test");

        // generate files in input directory
        generate_test_files(&input_dir, files).expect("Unable to generate the test files!");

        let final_input_dir = input_dir.clone().to_str().unwrap().to_string();
        let final_output_dir = output_dir.clone().to_str().unwrap().to_string();
        handle(
            format!(
                "sort --stack year --stack month {} {}",
                final_input_dir.clone(),
                final_output_dir.clone()
            ),
            Some(true),
        );
    }

    #[test]
    #[should_panic = "[ERROR] [Sort Command] 'tests/rsc/sort/test_4/input_dir/file' isn't a directory"]
    fn test_input_isnt_a_directory() {
        // define input & output directory
        let common_dir = &Path::new("tests").join("rsc").join("sort").join("test_4");
        let input_dir = common_dir.clone().join("input_dir").join("file");
        let output_dir = common_dir.clone().join("output_dir");

        clean_or_create_dir(output_dir.clone()).expect("should be able to create output_dir");

        let final_input_dir = input_dir.clone().to_str().unwrap().to_string();
        let final_output_dir = output_dir.clone().to_str().unwrap().to_string();

        handle(
            format!(
                "sort --stack year --stack month {} {}",
                final_input_dir.clone(),
                final_output_dir.clone()
            ),
            Some(true),
        );
    }

    #[test]
    #[should_panic = "[ERROR] [Sort Command] 'tests/rsc/sort/test_5/output_dir/file' isn't a directory"]
    fn test_output_isnt_a_directory() {
        // set filenames
        let files = vec![
            FileCreator::from("file_2022-02-22_F1BDD782"),
            FileCreator::from("file_2022-10-20_6FC02130"),
            FileCreator::from("file_2023-10-20_9E387272"),
        ];

        // define input & output directory
        let common_dir = &Path::new("tests").join("rsc").join("sort").join("test_5");
        let input_dir = common_dir.clone().join("input_dir");
        let output_dir = common_dir.clone().join("output_dir").join("file");

        clean_or_create_dir(input_dir.clone())
            .expect("Should be able to clean or create directory before running test");

        generate_test_files(&input_dir, files).expect("Unable to generate the test files!");

        let final_input_dir = input_dir.clone().to_str().unwrap().to_string();
        let final_output_dir = output_dir.clone().to_str().unwrap().to_string();
        handle(
            format!(
                "sort --stack year --stack month {} {}",
                final_input_dir.clone(),
                final_output_dir.clone()
            ),
            Some(true),
        );
    }
}
