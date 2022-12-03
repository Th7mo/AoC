pub fn read_file(project_folder_name: &str) -> String {
    let file_path = "bin/".to_string() + project_folder_name + "/src/input.txt";
    std::fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Can't read file {file_path}"))
}
