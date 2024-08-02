use ini::Ini;
use std::env;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Item {
    key: String,
    registry: String,
}
fn config_item(key: &str, registry: &str) -> Item {
    Item {
        key: key.to_string(),
        registry: registry.to_string(),
    }
}

#[derive(Debug)]
pub enum ConfigList {
    Fixed([Item; 2]),
    Dynamic(Vec<Item>),
}

fn init() -> ConfigList::Fixed {
    [
        config_item("npm", "https://registry.npmjs.org/"),
        config_item("taobao", "https://registry.npmmirror.com/"),
    ]
}
pub fn get() -> ConfigList::Dynamic {
    let init_list = init();
    let mut config_list = Vec::new();
    for i in init_list {
        config_list.push(i);
    }
    config_list
}
fn main() {
    let mut conf = Ini::new();
    let config_list = init();
    let mut registry_tab = conf.with_section(Some("registry"));
    for i in config_list {
        registry_tab.set(i.key, i.registry);
        // conf.with_section(Some(i.key)).set("registry", i.registry);
    }
    let root = Path::new("");
    // println!("root {}", root.display());
    let mut file_path = env::current_dir().unwrap();
    file_path.push("./.prmrc");
    // println!("file_path, {}", file_path.display());
    println!("file_path, {}", file_path.display());
    conf.write_to_file(file_path).unwrap();
}
