use slint::Model;
use slint::VecModel;
use zbus::blocking::ConnectionBuilder;
use zbus::interface;
use zbus::Result;
slint::include_modules!();
mod applications;
struct Utena;
#[interface(name = "org.revolution.utena")]
impl Utena {}
fn main() -> Result<()> {
    match ConnectionBuilder::session()?
        .name("org.revolution.utena")?
        .serve_at("/org/revolution/utena", Utena)?
        .build()
    {
        Ok(_) => {
            start_ui();
            Ok(())
        }
        Err(_) => {
            println!("Another session is running");
            Ok(())
        }
    }
}
fn start_ui() {
    let ui = AppWindow::new().unwrap();
    let apps_origin = applications::all_apps();
    let cats = applications::all_categrades(&apps_origin);
    let apps = std::rc::Rc::new(apps_origin);
    let apps2 = apps.clone();
    let apps_filter = apps.clone();
    let apps_filter2 = apps.clone();
    let image = ui.get_defaultimage();
    let image2 = image.clone();
    // apps
    {
        let vec = VecModel::default();
        vec.set_vec(
            apps.iter()
                .enumerate()
                .map(|(index, item)| MyItem {
                    title: slint::SharedString::from(item.title()),
                    index: index as i32,
                    description: slint::SharedString::from(item.description()),
                    icon: match item.icon() {
                        None => image.clone(),
                        Some(newimage) => newimage.clone(),
                    },
                    supported_types: {
                        let vec = VecModel::default();
                        vec.set_vec(
                            item.supported_types()
                                .iter()
                                .map(|unit| {
                                    slint::StandardListViewItem::from(slint::SharedString::from(
                                        unit.to_string(),
                                    ))
                                })
                                .collect::<Vec<slint::StandardListViewItem>>(),
                        );
                        slint::ModelRc::new(vec)
                        //vec.into()
                    },
                    categrades: {
                        let vec = VecModel::default();
                        if let Some(ref categrades) = item.categrades {
                            vec.set_vec(
                                categrades
                                    .iter()
                                    .map(slint::SharedString::from)
                                    .collect::<Vec<slint::SharedString>>(),
                            );
                        }
                        slint::ModelRc::new(vec)
                    },
                    hasactions: item.actions.is_some(),
                    actions: {
                        let vec = VecModel::default();
                        let actions = match &item.actions {
                            None => vec![],
                            Some(ref actions) => {
                                let mut action = vec![slint::SharedString::from("default")];
                                let mut others = actions
                                    .iter()
                                    .map(|anaction| slint::SharedString::from(anaction.to_string()))
                                    .collect::<Vec<slint::SharedString>>();
                                action.append(&mut others);
                                action
                            }
                        };
                        vec.set_vec(actions);
                        slint::ModelRc::new(vec)
                    },
                    actionchoosed: slint::SharedString::from("default"),
                })
                .collect::<Vec<MyItem>>(),
        );
        let model = std::rc::Rc::new(vec);
        ui.invoke_setItems(model.into());
    }

    // categrades
    {
        let vec = VecModel::default();
        vec.set_vec(
            cats.iter()
                .map(|cat| Cats {
                    cat: slint::SharedString::from(cat),
                    selected: false,
                })
                .collect::<Vec<Cats>>(),
        );
        let model = std::rc::Rc::new(vec);
        ui.set_cats(model.into());
    }

    ui.on_request_start_app(move |input: i32| {
        apps[input as usize].launch();
    });
    ui.on_request_start_app_with_action(move |index: i32, action: slint::SharedString| {
        apps2[index as usize].launch_with_action(&action);
    });
    let ui_handle = ui.as_weak();
    // fillter
    ui.on_request_fillter(move |input: slint::SharedString| {
        let ui = ui_handle.unwrap();
        let input = if regex::Regex::new(&input.to_lowercase()).is_err() {
            ui.invoke_reset_lineedit();
            "".to_string()
        } else {
            input.to_string()
        };
        let vec = VecModel::default();
        vec.set_vec(
            apps_filter
                .iter()
                .enumerate()
                .filter(|(_, item)| item.is_name_match(&input))
                .map(|(index, item)| MyItem {
                    title: slint::SharedString::from(item.title()),
                    index: index as i32,
                    description: slint::SharedString::from(item.description()),
                    icon: match item.icon() {
                        None => image.clone(),
                        Some(newimage) => newimage.clone(),
                    },
                    supported_types: {
                        let vec = VecModel::default();
                        vec.set_vec(
                            item.supported_types()
                                .iter()
                                .map(|unit| {
                                    slint::StandardListViewItem::from(slint::SharedString::from(
                                        unit.to_string(),
                                    ))
                                })
                                .collect::<Vec<slint::StandardListViewItem>>(),
                        );
                        slint::ModelRc::new(vec)
                        //vec.into()
                    },
                    categrades: {
                        let vec = VecModel::default();
                        if let Some(ref categrades) = item.categrades {
                            vec.set_vec(
                                categrades
                                    .iter()
                                    .map(slint::SharedString::from)
                                    .collect::<Vec<slint::SharedString>>(),
                            );
                        }
                        slint::ModelRc::new(vec)
                    },
                    hasactions: item.actions.is_some(),
                    actions: {
                        let vec = VecModel::default();
                        let actions = match &item.actions {
                            None => vec![],
                            Some(actions) => {
                                let mut action = vec![slint::SharedString::from("default")];
                                let mut others = actions
                                    .iter()
                                    .map(|anaction| slint::SharedString::from(anaction.to_string()))
                                    .collect::<Vec<slint::SharedString>>();
                                action.append(&mut others);
                                action
                            }
                        };
                        vec.set_vec(actions);
                        slint::ModelRc::new(vec)
                    },
                    actionchoosed: slint::SharedString::from("default"),
                })
                .collect::<Vec<MyItem>>(),
        );
        let model = std::rc::Rc::new(vec);
        ui.invoke_setItems(model.into());
    });
    let ui_handle = ui.as_weak();

    // fillter with categrades
    ui.on_request_fillter_with_cats(move |keyword, cats| {
        let cats = cats
            .iter()
            .filter(|cat| cat.selected)
            .map(|cat| cat.cat.to_string())
            .collect::<Vec<String>>();
        let ui = ui_handle.unwrap();
        let input = if regex::Regex::new(&keyword.to_lowercase()).is_err() {
            ui.invoke_reset_lineedit();
            "".to_string()
        } else {
            keyword.to_string()
        };
        let vec = VecModel::default();
        vec.set_vec(
            apps_filter2
                .iter()
                .enumerate()
                .filter(|(_, item)| item.is_name_match(&input) && item.is_incategrade(&cats))
                .map(|(index, item)| MyItem {
                    title: slint::SharedString::from(item.title()),
                    index: index as i32,
                    description: slint::SharedString::from(item.description()),
                    icon: match item.icon() {
                        None => image2.clone(),
                        Some(newimage) => newimage.clone(),
                    },
                    supported_types: {
                        let vec = VecModel::default();
                        vec.set_vec(
                            item.supported_types()
                                .iter()
                                .map(|unit| {
                                    slint::StandardListViewItem::from(slint::SharedString::from(
                                        unit.to_string(),
                                    ))
                                })
                                .collect::<Vec<slint::StandardListViewItem>>(),
                        );
                        slint::ModelRc::new(vec)
                        //vec.into()
                    },
                    categrades: {
                        let vec = VecModel::default();
                        if let Some(ref categrades) = item.categrades {
                            vec.set_vec(
                                categrades
                                    .iter()
                                    .map(slint::SharedString::from)
                                    .collect::<Vec<slint::SharedString>>(),
                            );
                        }
                        slint::ModelRc::new(vec)
                    },
                    hasactions: item.actions.is_some(),
                    actions: {
                        let vec = VecModel::default();
                        let actions = match &item.actions {
                            None => vec![],
                            Some(actions) => {
                                let mut action = vec![slint::SharedString::from("default")];
                                let mut others = actions
                                    .iter()
                                    .map(|anaction| slint::SharedString::from(anaction.to_string()))
                                    .collect::<Vec<slint::SharedString>>();
                                action.append(&mut others);
                                action
                            }
                        };
                        vec.set_vec(actions);
                        slint::ModelRc::new(vec)
                    },
                    actionchoosed: slint::SharedString::from("default"),
                })
                .collect::<Vec<MyItem>>(),
        );
        let model = std::rc::Rc::new(vec);
        ui.invoke_setItems(model.into());
    });
    ui.on_request_exit(|| {
        slint::quit_event_loop().unwrap();
    });
    ui.run().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
}
