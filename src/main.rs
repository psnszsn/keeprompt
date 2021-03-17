use keepass::Database;
use pinentry::PassphraseInput;
use secrecy::ExposeSecret;
use std::collections::HashMap;
use std::fs::File;

mod clipboard;
mod dmenu;
mod listener;
type PwdsHM = HashMap<String, keepass::Entry>;

#[derive(serde_derive::Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub pinentry: String,
    pub dmenu: String,
    pub database: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            pinentry: "pinentry-gnome3".into(),
            dmenu: "dmenu".into(),
            database: "".into(),
        }
    }
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("keeprompt").unwrap();
    let config_path = xdg_dirs
        .place_config_file("config.toml")
        .expect("cannot create configuration directory");

    let s = std::fs::read_to_string(config_path).unwrap();
    let config: Config = toml::from_str(s.as_str()).unwrap();

    println!("{:#?}", config);

    if let Ok(_) = listener::connect() {
        return;
    }
    let db = unlock_db(&config);
    let pwds = get_pwds(db);
    select_pwd(&config, &pwds);
    listener::run(&config, &pwds).unwrap();
}

fn select_pwd(config: &Config, pwds: &HashMap<String, keepass::Entry>) {
    let selected = dmenu::run(&pwds, config.dmenu.clone());
    // println!("{:#?}", selected);
    clipboard::copy(selected.get_password().unwrap());
}

pub fn unlock_db(config: &Config) -> Database {
    let passphrase = PassphraseInput::with_binary(&config.pinentry)
        .unwrap()
        .with_description("Enter KeePass passphrase")
        .with_prompt("Passphrase:")
        .interact()
        .unwrap();

    let path = std::path::Path::new(&config.database);
    let db = Database::open(
        &mut File::open(path).unwrap(),
        Some(passphrase.expose_secret()),
        None,
    )
    .unwrap();
    db
}

pub fn get_pwds(db: Database) -> HashMap<String, keepass::Entry> {
    fn map_group(group: keepass::Group) -> HashMap<String, keepass::Entry> {
        let group_name = group.name;
        let a: HashMap<String, keepass::Entry> = group
            .entries
            .into_iter()
            .map(|(k, v)| {
                let new_k = match group_name.as_str() {
                    "Root" => k,
                    x => format!("{}/{}", x.to_lowercase(), k),
                };

                (new_k, v)
            })
            .chain(
                group
                    .child_groups
                    .into_iter()
                    .map(|(_, g)| map_group(g))
                    .flatten(),
            )
            .collect();
        a
    };

    let all = map_group(db.root);
    // println!("{:#?}", all.keys());
    all
}
