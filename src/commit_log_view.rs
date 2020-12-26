use crate::commit_log::CommitLog;
use crate::commit_log_column::{CommitLogColumn, OriginalRow};
use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::gui_element_provider::GuiElementProvider;
use crate::source::Source;
use crate::tree_view::TreeView;
use crate::tree_view_column_config::{ColumnRenderer, TreeViewColumnConfig};

use gtk::TreeSelectionExt as _;
use gtk::TreeModelExt as _;
use std::cell::RefCell;
use std::convert::TryInto as _;
use std::rc::Rc;


pub struct CommitLogView
{
    commitLog: Rc<RefCell<CommitLog>>,
    sender: Sender
}

impl EventHandler for CommitLogView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::SelectionChanged(selection) => self.handleSelectionChanged(selection),
            _ => onUnknown(source, event)
        }
    }
}

impl CommitLogView
{
    pub fn new(commitLog: Rc<RefCell<CommitLog>>, guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        TreeView::new(
            guiElementProvider,
            "commitLogView",
            sender.clone(),
            Source::CommitLogViewWidget,
            makeColumnConfigs(sender.clone()));
        Self{commitLog, sender}
    }


    // private

    fn handleSelectionChanged(&self, selection: &gtk::TreeSelection)
    {
        match selection.get_selected() {
            Some((model, iter)) => {
                let row = model.get_value(&iter, CommitLogColumn::OriginalRow.into()).get_some::<OriginalRow>().unwrap()
                    .try_into().unwrap();
                let commitId = self.commitLog.borrow().getCommit(row).unwrap().id;
                self.sender.send((Source::CommitLogView, Event::CommitSelected(commitId))).unwrap();
            },
            None => self.sender.send((Source::CommitLogView, Event::CommitUnselected)).unwrap()
        }
    }
}

fn makeColumnConfigs(sender: Sender) -> Vec<TreeViewColumnConfig>
{
    let indexOfCheckButtonColumn = 0;
    let mut configs = vec![makeCheckButtonColumnConfig(indexOfCheckButtonColumn, sender)];
    for index in 1..=4 {
        configs.push(makeTextColumnConfig(index));
    }
    configs
}

fn makeCheckButtonColumnConfig(index: i32, sender: Sender) -> TreeViewColumnConfig
{
    TreeViewColumnConfig{
        index,
        renderer: ColumnRenderer::CheckButton(Box::new(move |_renderer, treePath| {
            sender.send((Source::CommitLogViewCheckButton, Event::MarkCommitForReportToggled(treePath))).unwrap();
        })),
        isResizable: false
    }
}

fn makeTextColumnConfig(index: i32) -> TreeViewColumnConfig
{
    TreeViewColumnConfig{index, renderer: ColumnRenderer::Text, isResizable: true}
}
