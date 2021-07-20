use crate::commit_diff::{makeCommitSummary, makeFormattedDiff};
use crate::diff_colorizer::DiffColorizer;
use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::gui_element_provider::GuiElementProvider;
use crate::repository::Repository;
use crate::source::Source;
use crate::text_view::TextView;

use std::rc::Rc;


pub struct CommitDiffView
{
    textView: TextView,
    diffColorizer: DiffColorizer,
    repository: Option<Rc<Repository>>
}

impl EventHandler for CommitDiffView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::CommitSelected(id)      => self.onCommitSelected(id),
            Event::CommitUnselected        => self.onCommitUnselected(),
            Event::RepositoryChanged(repo) => self.onRepositoryChanged(repo),
            Event::ZoomRequested(_)        => self.onZoomRequested(source, event),
            _ => onUnknown(source, event)
        }
    }
}

impl CommitDiffView
{
    pub fn new(repository: Option<Rc<Repository>>, guiElementProvider: &GuiElementProvider, sender: Sender) -> Self
    {
        let textView = TextView::new(guiElementProvider, "commitDiffView", sender, Source::CommitDiffViewWidget);
        let diffColorizer = DiffColorizer::new();
        diffColorizer.setupTextView(&textView);
        Self{
            textView,
            diffColorizer,
            repository
        }
    }


    // private

    fn onCommitSelected(&mut self, commitId: &git2::Oid)
    {
        match &self.repository {
            Some(repository) => {
                let commit = repository.findCommit(*commitId).unwrap();
                let commitTreesDiff = repository.makeDiffOfCommitAndParent(&commit);
                let textDiff = makeCommitSummary(&commit) + &makeFormattedDiff(&commitTreesDiff);
                self.diffColorizer.colorize(&self.textView, &textDiff);
            },
            None => panic!("Repository not set yet")
        }
    }

    fn onCommitUnselected(&self)
    {
        self.textView.clear();
    }

    fn onRepositoryChanged(&mut self, repo: &Rc<Repository>)
    {
        self.repository = Some(Rc::clone(repo));
    }

    fn onZoomRequested(&mut self, source: Source, event: &Event)
    {
        self.textView.handle(source, event);
    }
}
