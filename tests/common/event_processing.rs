pub fn processEvents()
{
    while gtk::events_pending() {
        gtk::main_iteration();
    }
}
