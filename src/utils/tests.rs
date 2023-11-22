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

    mod get_month_number {
        #[test]
        fn test_january() {
            todo!()
        }

        #[test]
        fn test_december() {
            todo!()
        }
    }

    #[test]
    fn test_get_year_number() {
        todo!()
    }

    mod move_file {
        #[test]
        fn test_move_file_when_directory_exists() {
            todo!()
        }

        #[test]
        #[should_panic]
        fn test_move_file_when_directory_dont_exists() {
            todo!()
        }

        #[test]
        fn test_move_file_and_create_missing_directories() {
            todo!()
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
}

mod time_manipulator_tests {
    use chrono::{DateTime, TimeZone, Utc};
    use std::time::SystemTime;

    use crate::utils::time_manipulator::{
        get_month_number_from_systemtime, get_year_number_from_systemtime,
    };

    #[test]
    fn get_year_number_from_systemtime_tests() {
        let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 0, 0, 0).unwrap().into();
        let system_time: SystemTime = date_time.into();
        assert_eq!(get_year_number_from_systemtime(system_time), 2014);
    }

    #[test]
    fn get_month_number_from_systemtime_tests_random_month() {
        // Random month
        let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 0, 0, 0).unwrap().into();
        let system_time: SystemTime = date_time.into();
        assert_eq!(get_month_number_from_systemtime(system_time), 7);
    }

    #[test]
    fn get_month_number_from_systemtime_tests_january_month() {
        // On the edge months
        let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 1, 8, 0, 0, 0).unwrap().into();
        let system_time: SystemTime = date_time.into();
        assert_eq!(get_month_number_from_systemtime(system_time), 1);
    }

    #[test]
    fn get_month_number_from_systemtime_tests_december_month() {
        let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 12, 8, 0, 0, 0).unwrap().into();
        let system_time: SystemTime = date_time.into();
        assert_eq!(get_month_number_from_systemtime(system_time), 12);
    }
}