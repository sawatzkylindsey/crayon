use crate::model::Page;
use crate::parameter::Parameters;
use std::fs::{copy, create_dir_all, read, read_dir, File};
use std::path::Path;
use walkdir::WalkDir;

mod constant;
mod model;
mod parameter;
mod templates;

fn main() {
    let parameters = parameter::parse();
    let template_engine = templates::load();

    create_dir_all(parameters.target_path.clone()).expect("must be able to create directories");
    copy_assets(&parameters);
    render_pages(&parameters, template_engine);
}

fn copy_assets(parameters: &Parameters) {
    for entry in WalkDir::new(&parameters.asset_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path: &Path = entry.path();

        if path.is_file() {
            let target_file = parameters.target_path.join(
                path.strip_prefix(&parameters.asset_path)
                    .expect("entry path must strip_prefix"),
            );
            copy(path, target_file).expect("must copy");
        } else {
            let target_dir = parameters.target_path.join(
                path.strip_prefix(&parameters.asset_path)
                    .expect("entry path must strip_prefix"),
            );
            create_dir_all(target_dir).expect("must be able to create directories");
        }
    }
}

fn render_pages(parameters: &Parameters, template_engine: templates::TemplateEngine) {
    for entry in read_dir(&parameters.content_path).expect("content_path must be readable") {
        let entry = entry.expect("entry must exist");
        let entry_content = read(entry.path()).expect("entry path must be readable");
        let page: Page =
            serde_yaml::from_slice(&entry_content).expect("entry path must deserialize as Page");
        let mut target_file = parameters
            .target_path
            .join(entry.path().file_stem().expect("must be able to file_stem"));
        target_file.set_extension("html");
        template_engine.render(
            File::create(target_file).expect("must be able to create file"),
            page,
        );
    }
}
