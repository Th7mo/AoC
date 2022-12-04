pub fn read_file(project_folder_name: &str) -> String {
   let file_path = get_file_path(project_folder_name);
    std::fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Can't read file {file_path}"))
}

fn get_file_path(project_folder_name: &str) -> String {
    format!("bin/{project_folder_name}/src/input.txt")
}
