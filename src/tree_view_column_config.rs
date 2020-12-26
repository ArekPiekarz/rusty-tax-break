pub struct TreeViewColumnConfig
{
    pub index: i32,
    pub renderer: ColumnRenderer,
    pub isResizable: bool,
}

pub enum ColumnRenderer
{
    Text,
    CheckButton(ToggledAction)
}

pub type ToggledAction = Box<dyn Fn(&gtk::CellRendererToggle, gtk::TreePath) + 'static>;
