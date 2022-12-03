pub fn read_file(project_folder_name: &str) -> String {
    let file_path = project_folder_name.to_string() + "/src/input.txt";
    std::fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Can't read file {file_path}"))
}
