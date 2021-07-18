use crate::event::Event;
use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;
use crate::tree_view_column_config::{ColumnRenderer, ToggledAction, TreeViewColumnConfig};

use gtk::prelude::CellLayoutExt as _;
use gtk::prelude::CellRendererToggleExt as _;
use gtk::prelude::TreeSelectionExt as _;
use gtk::prelude::TreeViewColumnExt as _;
use gtk::prelude::TreeViewExt as _;

const EXPAND_IN_LAYOUT : bool = true;


pub struct TreeView
{
    widget: gtk::TreeView,
}

impl TreeView
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        widgetName: &str,
        sender: Sender,
        source: Source,
        columnConfigs: Vec<TreeViewColumnConfig>)
        -> Self
    {
        let widget = guiElementProvider.get::<gtk::TreeView>(widgetName);
        let newSelf = Self{widget};
        newSelf.connectSelection(sender, source);
        newSelf.setupColumns(columnConfigs);
        newSelf
    }


    // private

    fn connectSelection(&self, sender: Sender, eventSource: Source)
    {
        self.widget.selection().connect_changed(move |selection|
            sender.send((eventSource, Event::SelectionChanged(selection.clone()))).unwrap());
    }

    fn setupColumns(&self, columnConfigs: Vec<TreeViewColumnConfig>)
    {
        for config in columnConfigs {
            match config.renderer {
                ColumnRenderer::Text => self.setupTextColumn(config),
                ColumnRenderer::CheckButton(toggledAction) => self.setupCheckButtonColumn(
                    config.index, toggledAction, config.isResizable)
            }
        }
    }

    fn setupTextColumn(&self, columnConfig: TreeViewColumnConfig)
    {
        let renderer = gtk::CellRendererText::new();
        let index = columnConfig.index;
        let column = self.widget.column(index).unwrap();
        column.pack_start(&renderer, EXPAND_IN_LAYOUT);
        column.add_attribute(&renderer, "text", index);
        column.set_resizable(columnConfig.isResizable);
        column.set_reorderable(true);
    }

    fn setupCheckButtonColumn(&self, index: i32, toggledAction: ToggledAction, isResizable: bool)
    {
        let renderer = gtk::CellRendererToggle::new();
        renderer.connect_toggled(move |renderer, treePath| { toggledAction(renderer, treePath); });
        let column = self.widget.column(index).unwrap();
        column.pack_start(&renderer, EXPAND_IN_LAYOUT);
        column.add_attribute(&renderer, "active", index);
        column.set_resizable(isResizable);
        column.set_reorderable(true);
    }
}
