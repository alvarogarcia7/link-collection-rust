pub mod fzf_selector_mod {
    //     fn select_multiple_from(colours: Vec<&str>) -> _ {
    //         let mut fzf = Fzf::builder()
    //             .custom_args(vec!["--multi".to_string()])
    //             .build()
    //             .unwrap();
    //
    //         fzf.run().expect("Failed to start fzf");
    //
    //         fzf.add_items(colours).expect("Failed to add items");
    //
    //         let users_selection = fzf.output().expect("Failed to get the user's output");
    //
    //         let tags = users_selection.split("\n").collect::<Vec<&str>>();
    //
    //         // println!("User selected: {:?}", tags);
    //         tags
    //     }
    use fzf_wrapped::Fzf;

    pub struct FzfSelector {}

    impl FzfSelector {
        pub fn select_multiple_from(
            prompt: &str,
            items: impl IntoIterator<Item = impl Into<String>>,
        ) -> Vec<String> {
            let mut fzf = Fzf::builder()
                // .multi(true) // This does not work, need to use custom_args - hallucination of the LLM
                .custom_args(vec!["--multi".to_string(), "--print-query".to_string()])
                .prompt(prompt)
                .build()
                .unwrap();

            fzf.run()
                .expect("Failed to start fzf: is it present in the $PATH?");

            let mut sorted_items: Vec<String> = items.into_iter().map(|item| item.into()).collect();
            sorted_items.sort();

            fzf.add_items(sorted_items.iter().rev()).unwrap();

            let output = fzf.output();

            match output {
                Some(selected) => selected
                    .lines()
                    .map(|line| line.trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect(),
                None => vec![],
            }
        }

        pub fn select_single_from(
            prompt: &str,
            items: impl IntoIterator<Item = impl Into<String>>,
        ) -> Vec<String> {
            let mut fzf = Fzf::builder().prompt(prompt).build().unwrap();

            fzf.run()
                .expect("Failed to start fzf: is it present in the $PATH?");

            let mut sorted_items: Vec<String> = items.into_iter().map(|item| item.into()).collect();
            sorted_items.sort();

            fzf.add_items(sorted_items.iter().rev()).unwrap();

            let output = fzf.output();

            match output {
                Some(selected) => selected
                    .lines()
                    .map(|line| line.trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect(),
                None => vec![],
            }
        }
    }
}
