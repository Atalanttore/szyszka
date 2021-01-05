use crate::class_gui_data::GuiData;
use crate::create_tree_view::{create_tree_view_results, create_tree_view_rules};
use glib::*;
use gtk::*;

pub fn initialize_gui(gui_data: &mut GuiData) {
    // Create TreeView in Scrolled Window
    {
        let scrolled_window_results: ScrolledWindow = gui_data.results.scrolled_window_results.clone();

        let col_types: [Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];

        let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

        let tree_view: gtk::TreeView = TreeView::with_model(&list_store);

        tree_view.get_selection().set_mode(SelectionMode::Multiple);

        create_tree_view_results(&tree_view);

        scrolled_window_results.add(&tree_view);
        scrolled_window_results.show_all();

        gui_data.results.tree_view_results = tree_view;
    }
    // Create TreeView in Rules
    {
        let scrolled_window_rules: ScrolledWindow = gui_data.rules_bottom_panel.scrolled_window_rules.clone();

        let col_types: [Type; 3] = [glib::types::Type::U32, glib::types::Type::String, glib::types::Type::String];

        let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

        let tree_view: gtk::TreeView = TreeView::with_model(&list_store);

        tree_view.get_selection().set_mode(SelectionMode::Multiple);

        create_tree_view_rules(&tree_view);

        scrolled_window_rules.add(&tree_view);
        scrolled_window_rules.show_all();

        gui_data.rules_bottom_panel.tree_view_window_rules = tree_view;
    }
}
