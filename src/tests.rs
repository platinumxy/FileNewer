#[cfg(test)]
mod testing_file_manager{
    #[test]
    fn evaluate_path_vars() {
        let epv =|vr:&str| { // epv - evaluate path value
            crate::file_manager::evaluate_path_vars(vr).unwrap()
        };
        let test_eq = |in_str:&str, out_str:String | {
                assert_eq!(epv(in_str), out_str.to_owned());
            };

        let user_pth = |ext: &str| -> String {
            let mut user_profile = std::env::var("USERPROFILE").unwrap();
            user_profile.push_str(ext);
            user_profile};

        test_eq("~", user_pth("\\"));
        test_eq("~/Some/Path", user_pth("\\Some\\Path\\"));
        test_eq("~\\Some\\Path\\", user_pth("\\Some\\Path\\"));
        test_eq("~\\\\Some\\Path\\", user_pth("\\Some\\Path\\"));
        test_eq("~\\", user_pth("\\"));
        test_eq("%USERPROFILE%", user_pth("\\"));
        test_eq("/users", "C:\\users\\".to_string());
        test_eq("/users\\public", "C:\\users\\public\\".to_string());
    }
}