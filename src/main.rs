use slint::VecModel;

slint::include_modules!();
mod applications;
fn main() {
    let ui = AppWindow::new();
    let apps = applications::all_apps();
    let apps = std::rc::Rc::new(apps);
    let apps_filter = apps.clone();
    let vec = VecModel::default();
    vec.set_vec(
        apps.iter()
            .enumerate()
            .map(|(index, item)| MyItem {
                title: slint::SharedString::from(item.title()),
                index: index as i32,
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
        let vec = VecModel::default();
        vec.set_vec(
            apps_filter
                .iter()
                .enumerate()
                .filter(|(_, item)| item.is_name_match(&input))
                .map(|(index, item)| MyItem {
                    title: slint::SharedString::from(item.title()),
                    index: index as i32,
                })
                .collect::<Vec<MyItem>>(),
        );
        let model = std::rc::Rc::new(vec);
        let ui = ui_handle.unwrap();
        ui.set_items(model.into());
    });
    ui.on_request_exit(|| {
        slint::quit_event_loop().unwrap();
    });
    ui.run();
}
