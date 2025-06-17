mod file_manipulator_tests {

    mod to_absolute_path {
        use std::env;

        use crate::utils::file_manipulator::to_absolute_path;

        #[test]
        fn test_already_absolute_path() {
            let test_dir_pathbuf = env::current_dir().unwrap().join("test-dir");
            let current_dir_str = test_dir_pathbuf.as_os_str().to_str().unwrap();
            let path_string = String::from(current_dir_str);
            let transformed_absolute_path = to_absolute_path(path_string.clone());

            assert_eq!(transformed_absolute_path, path_string);
        }

        #[test]
        fn test_relative_path() {
            let path_string = String::from("test-dir");
            let expected_pathbuf = env::current_dir().unwrap().join(path_string.clone());
            let expected_path_string = expected_pathbuf.as_os_str().to_str().unwrap();
            let transformed_absolute_path = to_absolute_path(path_string.clone());

            assert_eq!(transformed_absolute_path, expected_path_string);
        }
    }

    mod get_last_modified_time {
        use std::{env, fs::File};

        use chrono::Datelike;
        use rsft_utils::{common::generate_test_files, file_creator::FileCreator};

        use crate::utils::file_manipulator::get_last_modified_time;

        #[test]
        fn test_january() {
            let filename = "file_2021-01-08_F71C2883";
            let target_dir = env::current_dir()
                .unwrap()
                .join("tests")
                .join("rsc")
                .join("files");
            let full_path = target_dir.clone().join(filename);

            let file_creator = FileCreator::from(filename);
            generate_test_files(&target_dir, vec![file_creator])
                .expect("Should be able to generate tests files");

            let file = File::open(full_path).expect("Should be able to access file, please check that 'tests/rsc/files/file_2021-01-08_F71C2883' is present");
            let datetime =
                get_last_modified_time(&file).expect("Should be able to extract month number");

            assert_eq!(datetime.month0(), 0u32);
        }

        #[test]
        fn test_december() {
            let filename = "file_2021-12-06_F4E4926F";
            let target_dir = env::current_dir()
                .unwrap()
                .join("tests")
                .join("rsc")
                .join("files");
            let full_path = target_dir.clone().join(filename);

            let file_creator = FileCreator::from(filename);
            generate_test_files(&target_dir, vec![file_creator])
                .expect("Should be able to generate tests files");

            let file = File::open(full_path).expect("Should be able to access file, please check that 'tests/rsc/files/file_2021-12-06_F4E4926F' is present");
            let datetime =
                get_last_modified_time(&file).expect("Should be able to extract month number");

            assert_eq!(datetime.month0(), 11u32);
        }

        #[test]
        fn test_get_year_number() {
            let filename = "file_2018-11-06_14835535";
            let target_dir = env::current_dir()
                .unwrap()
                .join("tests")
                .join("rsc")
                .join("files");

            let full_path = target_dir.clone().join(filename);

            let file_creator = FileCreator::from(filename);
            generate_test_files(&target_dir, vec![file_creator])
                .expect("Should be able to generate tests files");

            let file = File::open(full_path).expect("Should be able to access file, please check that 'tests/rsc/files/file_2018-11-06_14835535' is present");
            let datetime =
                get_last_modified_time(&file).expect("Should be able to extract year number");

            assert_eq!(datetime.year(), 2018);
        }
    }

    mod move_file {
        use std::{env, fs};

        use crate::utils::file_manipulator::move_file;

        #[test]
        fn test_move_file_when_directory_exists() {
            let filename = "test_move_file_1";
            let from = env::current_dir()
                .unwrap()
                .join("tests")
                .join("rsc")
                .join("files")
                .join(filename);
            let to = env::current_dir()
                .unwrap()
                .join("tests")
                .join("rsc")
                .join("files")
                .join("output")
                .join(filename);

            // create file
            let _ = fs::write(from.clone(), "my content");

            // function to test
            match move_file(from, to.clone(), false) {
                Err(_) => {
                    // fail
                    assert!(false)
                }
                Ok(_) => (),
            };

            match fs::read_to_string(to.clone()) {
                Ok(content) => {
                    // check that right file have be copy
                    assert_eq!(content.as_str(), "my content");
                    fs::remove_file(to).expect("Should be able to clean output directory");
                }
                Err(_) => {
                    // fail
                    assert!(false)
                }
            }
        }

        #[test]
        fn test_move_file_when_directory_dont_exists() {
            let filename = "test_move_file_2";
            let from = env::current_dir()
                .unwrap()
                .join("tests")
                .join("rsc")
                .join("files")
                .join(filename);
            let to = env::current_dir()
                .unwrap()
                .join("tests")
                .join("rsc")
                .join("files")
                .join("output")
                .join("output_unknown_1")
                .join(filename);

            // create file
            let _ = fs::write(from.clone(), "my content");

            // function to test
            match move_file(from.clone(), to.clone(), false) {
                Err(_) => {
                    fs::remove_file(from).expect("Should be able to delete test file");
                    assert!(true)
                }
                Ok(_) => {
                    fs::remove_file(from).expect("Should be able to delete test file");
                    // fail
                    assert!(false)
                }
            };
        }

        #[test]
        fn test_move_file_and_create_missing_directories() {
            let filename = "test_move_file_3";
            let from = env::current_dir()
                .unwrap()
                .join("tests")
                .join("rsc")
                .join("files")
                .join(filename);
            let to_dir = env::current_dir()
                .unwrap()
                .join("tests")
                .join("rsc")
                .join("files")
                .join("output")
                .join("output_unknown_2");
            let to = to_dir.clone().join(filename);

            // create file
            let _ = fs::write(from.clone(), "my content");

            // function to test
            match move_file(from, to.clone(), true) {
                Err(_) => {
                    // fail
                    assert!(false)
                }
                Ok(_) => (),
            };

            match fs::read_to_string(to.clone()) {
                Ok(content) => {
                    // check that right file have be copy
                    assert_eq!(content.as_str(), "my content");
                    fs::remove_dir_all(to_dir).expect("Should be able to clean output directory");
                }
                Err(_) => {
                    // fail
                    assert!(false)
                }
            }
        }
    }
}

mod string_manipulator_tests {
    mod add_0_to_single_number {

        use crate::utils::string_manipulator::add_0_to_single_number;

        #[test]
        fn test_add_0_when_input_0() {
            let value = add_0_to_single_number(0);
            assert_eq!(value, String::from("00"));
        }
        #[test]
        fn test_add_0_when_input_lower_than_10() {
            let value = add_0_to_single_number(9);
            assert_eq!(value, String::from("09"));
        }

        #[test]
        fn test_dont_add_0_when_input_more_than_10() {
            let value = add_0_to_single_number(10);
            assert_eq!(value, String::from("10"));
        }

        #[test]
        fn test_dont_limit_to_2_length_string() {
            let value = add_0_to_single_number(100);
            assert_eq!(value, String::from("100"));
        }
    }

    mod random_string {
        use crate::utils::string_manipulator::random_string;

        #[test]
        fn test_random_string() {
            let value = random_string(10);
            assert_eq!(value.len(), 10);
        }

        #[test]
        fn test_random_string_0_length() {
            let value = random_string(0);
            assert_eq!(value.len(), 0);
        }
    }
}
