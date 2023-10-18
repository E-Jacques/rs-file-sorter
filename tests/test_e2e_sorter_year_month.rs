mod common;

#[cfg(test)]
pub mod test_e2e_sorter_year_month {
    use crate::common::clean_dir;

    use super::common::file_or_dir_exists;
    use rs_file_sorter::sortering_strategies::year_month_sotring_strategy::get_year_month_sorting_strategy;
    use std::{fs, path::Path};

    #[test]
    fn test_sort_to_same_file() {
        let files = vec![
            "file_2022-02-22_F1BDD782",
            "file_2022-10-20_6FC02130",
            "file_2023-10-20_9E387272",
        ];
        let test_dir = Path::new("tests").join("rsc").join("sort").join("test_1");
        let original_dir = test_dir.clone().join("original_dir");
        let target_dir = test_dir.clone().join("target_dir");

        clean_dir(target_dir.clone())
            .expect("Should be able to clean directory before running test.");
        
        for filename in files {
            let from = original_dir.clone().join(filename);
            let to = target_dir.clone().join(filename);
            fs::copy(from.clone(), to.clone()).expect(
                format!(
                    "Trying to copy {} to {}. Cannot run test if cannot copy files.",
                    from.to_str().unwrap(),
                    to.to_str().unwrap()
                )
                .as_str(),
            );
        }

        let year_month_sorting_strategy = get_year_month_sorting_strategy();
        rs_file_sorter::sorter::sorter(
            target_dir.clone().to_str().unwrap(),
            target_dir.clone().to_str().unwrap(),
            year_month_sorting_strategy,
            |from, to| {
                assert!(false, "Cannot move {from} to {to}.");
            },
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
