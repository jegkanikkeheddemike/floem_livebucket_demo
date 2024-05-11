use std::{ops::Deref, rc::Rc};

use floem::{
    reactive::create_rw_signal,
    views::{button, dyn_container, stack_from_iter, text_input, Decorators},
    IntoView,
};
use floem_livebucket_integration::watch;

fn main() {
    floem::launch(app);
}

fn app() -> impl IntoView {
    let conn = Rc::new(livebucket::client::LVBClient::new("jensogkarsten.site"));

    let newkey = create_rw_signal(String::new());

    (
        watch(conn.deref(), [""], |[all_items]| {
            dyn_container(move || {
                let res = all_items.get();
                if let Some(data) = res {
                    stack_from_iter(data.into_iter().map(|p| p.key))
                        .style(|s| s.flex_col())
                        .into_any()
                } else {
                    "Loading!".into_any()
                }
            })
            .style(|s| s.justify_center().items_center())
        }),
        (
            text_input(newkey),
            button(|| "create more!").on_click(move |_| {
                conn.insert(newkey.get().as_str(), "value");
                floem::event::EventPropagation::Stop
            }),
        ),
    )
        .style(|s| s.flex_col().justify_center().size_full())
}
