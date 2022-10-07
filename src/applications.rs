use gio::prelude::*;
use gio::{AppInfo, AppLaunchContext};
//use once_cell::sync::Lazy;

#[allow(dead_code)]
pub struct App {
    appinfo: AppInfo,
    name: String,
    discriptions: Option<gio::glib::GString>,
    categrades: Option<Vec<String>>,
}
impl App {
    pub fn launch(&self) {
        self.appinfo.launch(&[], None::<&AppLaunchContext>).unwrap();
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
    pub fn is_name_match(&self, input: &str) -> bool {
        let re = regex::Regex::new(&input.to_lowercase()).unwrap();
        re.is_match(&self.name.to_lowercase()) || {
            match &self.discriptions {
                None => false,
                Some(description) => re.is_match(description),
            }
        }
    }
    pub fn title(&self) -> &str {
        &self.name
    }
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
        .map(|app| App {
            appinfo: app.clone(),
            name: app.name().to_string(),
            discriptions: app.description(),
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
