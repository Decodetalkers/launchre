use gio::prelude::*;
use gio::{AppInfo, AppLaunchContext};
//use once_cell::sync::Lazy;
use slint::Image;
#[allow(dead_code)]
pub struct App {
    appinfo: AppInfo,
    name: String,
    descriptions: Option<gio::glib::GString>,
    pub categrades: Option<Vec<String>>,
    icon: Option<Image>,
}
impl App {
    pub fn launch(&self) {
        if let Err(err) = self.appinfo.launch(&[], None::<&AppLaunchContext>) {
            println!("{}", err);
        };
        slint::quit_event_loop().unwrap();
    }
    #[allow(dead_code)]
    pub fn is_incategrade(&self, cats: Vec<&str>) -> bool {
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
        re.is_match(&self.name.to_lowercase()) || {
            match &self.descriptions {
                None => false,
                Some(description) => re.is_match(&description.to_lowercase()),
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
fn get_icon_path(iconname: &str) -> Option<Image> {
    if iconname.contains('/') {
        let path = std::path::Path::new(iconname);
        return match Image::load_from_path(path) {
            Ok(image) => Some(image),
            Err(_) => None,
        };
    }
    let svg = format!("/usr/share/icons/hicolor/scalable/apps/{}.svg", iconname);
    let svgpath = std::path::Path::new(&svg);
    if svgpath.exists() {
        return match Image::load_from_path(svgpath) {
            Ok(image) => Some(image),
            Err(_) => None,
        };
    }

    let paths = ["256x256", "128x128"];
    for path in paths {
        let icon = format!("/usr/share/icons/hicolor/{}/apps/{}.png", path, iconname);
        let iconpath = std::path::Path::new(&icon);
        if iconpath.exists() {
            return match Image::load_from_path(iconpath) {
                Ok(image) => Some(image),
                Err(_) => None,
            };
        }
    }
    let pixsvg = format!("/usr/share/pixmaps/{}.svg", iconname);
    let pixpath = std::path::Path::new(&pixsvg);
    if pixpath.exists() {
        return match Image::load_from_path(pixpath) {
            Ok(image) => Some(image),
            Err(_) => None,
        };
    }
    let pixpng = format!("/usr/share/pixmaps/{}.png", iconname);
    let pixpath = std::path::Path::new(&pixpng);
    if pixpath.exists() {
        return match Image::load_from_path(pixpath) {
            Ok(image) => Some(image),
            Err(_) => None,
        };
    }
    None
}
#[allow(dead_code)]
pub fn all_categrades(apps: Vec<App>) -> Vec<String> {
    let mut cats = vec![];
    for app in apps {
        if let Some(cat) = app.categrades {
            for acat in cat {
                if !cats.contains(&acat) {
                    cats.push(acat);
                }
            }
        }
    }
    cats
}
pub fn all_apps() -> Vec<App> {
    let re = regex::Regex::new(r"([a-zA-Z]+);").unwrap();
    gio::AppInfo::all()
        .iter()
        .filter(|app| app.should_show())
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
