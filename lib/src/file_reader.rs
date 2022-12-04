pub fn file_in_lines(project_folder_name: &str) -> String {
    let file_path = get_file_path(project_folder_name);

    let Ok(file_as_string) = std::fs::read_to_string(&file_path) else {
        panic!("can't read file {file_path}");
    };

    file_as_string
}

fn get_file_path(project_folder_name: &str) -> String {
    format!("bin/{project_folder_name}/src/input.txt")
}
