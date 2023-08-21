use std::path::PathBuf;

use gio::prelude::*;
use gio::{AppLaunchContext, DesktopAppInfo};
//use once_cell::sync::Lazy;
use slint::Image;
pub struct App {
    appinfo: DesktopAppInfo,
    name: String,
    descriptions: Option<gio::glib::GString>,
    pub categrades: Option<Vec<String>>,
    pub actions: Option<Vec<gio::glib::GString>>,
    icon: Option<Image>,
}
impl App {
    pub fn launch(&self) {
        if let Err(err) = self.appinfo.launch(&[], AppLaunchContext::NONE) {
            println!("{}", err);
        };

        slint::quit_event_loop().unwrap();
    }
    pub fn launch_with_action(&self, action: &str) {
        self.appinfo.launch_action(action, AppLaunchContext::NONE);

        slint::quit_event_loop().unwrap();
    }
    // if is in categrade
    pub fn is_incategrade(&self, cats: &Vec<String>) -> bool {
        match self.categrades {
            None => false,
            Some(ref categrade) => {
                for cat in cats {
                    if !categrade.contains(&cat.to_string()) {
                        return false;
                    };
                }
                true
            }
        }
    }
    pub fn is_name_match<T>(&self, input: T) -> bool
    where
        T: ToString,
    {
        let re = regex::Regex::new(&input.to_string().to_lowercase()).unwrap();
        re.is_match(&self.name.to_lowercase())
            || re.is_match(&deunicode::deunicode(&self.name).to_lowercase())
            || {
                match &self.descriptions {
                    None => false,
                    Some(description) => {
                        re.is_match(&description.to_lowercase())
                            || re.is_match(&deunicode::deunicode(description).to_lowercase())
                    }
                }
            }
    }
    pub fn title(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        match &self.descriptions {
            None => "",
            Some(description) => description,
        }
    }
    pub fn icon(&self) -> &Option<Image> {
        &self.icon
    }
    pub fn supported_types(&self) -> Vec<gio::glib::GString> {
        self.appinfo.supported_types()
    }
}

static ICONS_SIZE: &[&str] = &["256x256", "128x128"];

static THEMES_LIST: &[&str] = &["breeze", "Adwaita"];

fn get_icon_path_from_xdgicon(iconname: &str) -> Option<PathBuf> {
    let scalable_icon_path =
        xdg::BaseDirectories::with_prefix("icons/hicolor/scalable/apps").unwrap();
    if let Some(iconpath) = scalable_icon_path.find_data_file(format!("{iconname}.svg")) {
        return Some(iconpath);
    }
    for prefix in ICONS_SIZE {
        let iconpath =
            xdg::BaseDirectories::with_prefix(&format!("icons/hicolor/{prefix}/apps")).unwrap();
        if let Some(iconpath) = iconpath.find_data_file(format!("{iconname}.png")) {
            return Some(iconpath);
        }
    }
    let pixmappath = xdg::BaseDirectories::with_prefix("pixmaps").unwrap();
    if let Some(iconpath) = pixmappath.find_data_file(format!("{iconname}.svg")) {
        return Some(iconpath);
    }
    if let Some(iconpath) = pixmappath.find_data_file(format!("{iconname}.png")) {
        return Some(iconpath);
    }
    for themes in THEMES_LIST {
        let iconpath =
            xdg::BaseDirectories::with_prefix(&format!("icons/{themes}/apps/48")).unwrap();
        if let Some(iconpath) = iconpath.find_data_file(format!("{iconname}.svg")) {
            return Some(iconpath);
        }
        let iconpath =
            xdg::BaseDirectories::with_prefix(&format!("icons/{themes}/apps/64")).unwrap();
        if let Some(iconpath) = iconpath.find_data_file(format!("{iconname}.svg")) {
            return Some(iconpath);
        }
    }
    None
}

fn get_icon_path(iconname: &str) -> Option<Image> {
    if iconname.contains('/') {
        let path = std::path::Path::new(iconname);
        return match Image::load_from_path(path) {
            Ok(image) => Some(image),
            Err(_) => None,
        };
    }
    if let Some(pixpath) = get_icon_path_from_xdgicon(iconname) {
        return match Image::load_from_path(&pixpath) {
            Ok(image) => Some(image),
            Err(_) => None,
        };
    }
    None
}
// return categrade
pub fn all_categrades(apps: &Vec<App>) -> Vec<String> {
    let mut cats: Vec<String> = vec![];
    for app in apps {
        if let Some(cat) = &app.categrades {
            for acat in cat {
                if !cats.contains(acat) {
                    cats.push(acat.to_string());
                }
            }
        }
    }
    cats
}
// return all apps
pub fn all_apps() -> Vec<App> {
    let re = regex::Regex::new(r"([a-zA-Z]+);").unwrap();
    gio::AppInfo::all()
        .iter()
        .filter(|app| app.should_show() && app.downcast_ref::<gio::DesktopAppInfo>().is_some())
        .map(|app| app.clone().downcast::<gio::DesktopAppInfo>().unwrap())
        .map(|app| App {
            appinfo: app.clone(),
            name: app.name().to_string(),
            descriptions: app.description(),
            categrades: match app.clone().downcast::<gio::DesktopAppInfo>() {
                Err(_) => None,
                Ok(item) => {
                    match item.categories() {
                        None => None,
                        Some(categrades) => {
                            let tomatch = categrades.to_string();
                            let tips = re
                                .captures_iter(&tomatch)
                                .map(|unit| unit.get(1).unwrap().as_str().to_string())
                                .collect();
                            Some(tips)
                        }
                    }
                    //None
                }
            },
            actions: match app.clone().downcast::<gio::DesktopAppInfo>() {
                Err(_) => None,
                Ok(item) => {
                    let actions = item.list_actions();
                    if actions.is_empty() {
                        None
                    } else {
                        Some(actions)
                    }
                } //None
            },
            icon: match &app.icon() {
                None => None,
                Some(icon) => {
                    let iconname = gio::prelude::IconExt::to_string(icon).unwrap();
                    get_icon_path(iconname.as_str())
                }
            },
        })
        .collect()
}

#[test]
fn split() {
    let re = regex::Regex::new(r"([a-zA-Z]+);").unwrap();
    let tomatch = "Categrade;beta;";
    let tips: Vec<&str> = re
        .captures_iter(tomatch)
        .map(|unit| unit.get(1).unwrap().as_str())
        .collect();
    assert_eq!(vec!["Categrade", "beta"], tips);
}

#[test]
fn unicode() {
    let re = regex::Regex::new("ce shi").unwrap();
    println!("{}", deunicode::deunicode("測試"));
    assert!(re.is_match(&deunicode::deunicode("測試").to_lowercase()));
}
