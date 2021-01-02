use crate::common::event_processing::processEvents;

use glib::Cast as _;
use gtk::GtkWindowExt as _;


pub struct TestGui
{
    window: gtk::ApplicationWindow
}

impl TestGui
{
    pub fn new(window: gtk::ApplicationWindow) -> Self
    {
        Self{window}
    }

    pub fn findWidget<T>(&self, name: &str) -> T
        where T: glib::IsA<gtk::Widget>
    {
        gtk_test::find_child_by_name::<T, gtk::ApplicationWindow>(&self.window, name).unwrap()
    }
}

impl Drop for TestGui
{
    fn drop(&mut self)
    {
        for widget in &gtk::Window::list_toplevels() {
            let window = widget.downcast_ref::<gtk::Window>().unwrap();
            window.close();
            processEvents();
        }
    }
}
