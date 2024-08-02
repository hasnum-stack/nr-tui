use ini::Ini;
use std::env;
use std::path::Path;
struct Config {
    key: String,
    registry: String,
}
fn config_item(key: &str, registry: &str) -> Config {
    Config {
        key: key.to_string(),
        registry: registry.to_string(),
    }
}
fn init_config() -> [Config; 2] {
    [
        config_item("npm", "https://registry.npmjs.org/"),
        config_item("taobao", "https://registry.npmmirror.com/"),
    ]
}
fn main() {
    let mut conf = Ini::new();
    let config_list = init_config();
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
