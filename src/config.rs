use directories::ProjectDirs;

pub fn get_data_path() -> std::path::PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("", "KhiemNguyen15", "todo") {
        let data_dir = proj_dirs.data_local_dir();
        std::fs::create_dir_all(data_dir).unwrap();

        return data_dir.join("todo.db");
    }

    panic!("Could not determine data directory");
}

pub fn remove_data_dir() {
    if let Some(proj_dirs) = ProjectDirs::from("", "KhiemNguyen15", "todo") {
        let data_dir = proj_dirs.data_local_dir();
        std::fs::remove_dir_all(data_dir).unwrap();
    }
}
