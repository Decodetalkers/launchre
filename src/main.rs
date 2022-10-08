use slint::VecModel;

slint::include_modules!();
mod applications;
fn main() {
    let ui = AppWindow::new();
    let apps = applications::all_apps();
    let apps = std::rc::Rc::new(apps);
    let apps_filter = apps.clone();
    let image = ui.get_defaultimage();
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
            })
            .collect::<Vec<MyItem>>(),
    );
    let model = std::rc::Rc::new(vec);

    ui.set_items(model.into());
    ui.on_request_start_app(move |input: i32| {
        apps[input as usize].launch();
    });
    let ui_handle = ui.as_weak();
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
                })
                .collect::<Vec<MyItem>>(),
        );
        let model = std::rc::Rc::new(vec);
        ui.set_items(model.into());
    });
    ui.on_request_exit(|| {
        slint::quit_event_loop().unwrap();
    });
    ui.run();
}
