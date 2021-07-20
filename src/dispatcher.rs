use crate::channel::attach;
use crate::choose_folder_button::ChooseFolderButton;
use crate::commit_diff_view::CommitDiffView;
use crate::commit_log::CommitLog;
use crate::commit_log_model::CommitLogModel;
use crate::commit_log_model_filter::CommitLogModelFilter;
use crate::commit_log_view::CommitLogView;
use crate::config_store::ConfigStore;
use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown, Receiver};
use crate::options_dialog::OptionsDialog;
use crate::output_path_label::OutputPathLabel;
use crate::output_path_store::OutputPathStore;
use crate::report_generator::ReportGenerator;
use crate::repository_path_label::RepositoryPathLabel;
use crate::repository_store::RepositoryStore;
use crate::source::Source;

use gtk::glib;
use std::cell::RefCell;
use std::rc::Rc;

pub fn setupDispatching(handlers: EventHandlers, receiver: Receiver)
{
    let mut chooseOutputFolderButton = handlers.chooseOutputFolderButton;
    let mut chooseRepositoryFolderButton = handlers.chooseRepositoryFolderButton;
    let mut commitDiffView = handlers.commitDiffView;
    let mut commitLog = handlers.commitLog;
    let mut commitLogModelFilter = handlers.commitLogFilterModel;
    let mut commitLogModel = handlers.commitLogModel;
    let mut commitLogView = handlers.commitLogView;
    let mut configStore = handlers.configStore;
    let mut optionsDialog = handlers.optionsDialog;
    let mut outputPathLabel = handlers.outputPathLabel;
    let mut outputPathStore = handlers.outputPathStore;
    let mut repositoryStore = handlers.repositoryStore;
    let mut repositoryPathLabel = handlers.repositoryPathLabel;
    let mut reportGenerator = handlers.reportGenerator;

    use Source as S;
    use Event as E;
    attach(receiver, move |(source, event)| { match (source, &event) {
        (S::ChooseOutputFolderButtonWidget,     E::Clicked)                          => chooseOutputFolderButton.handle(source, &event),
        (S::ChooseOutputFolderButton,           E::FolderChosen(_))                  => outputPathStore.handle(source, &event),
        (S::ChooseOutputFolderDialog,           E::DialogResponded(_))               => chooseOutputFolderButton.handle(source, &event),
        (S::ChooseRepositoryFolderButton,       E::FolderChosen(_))                  => repositoryStore.handle(source, &event),
        (S::ChooseRepositoryFolderButtonWidget, E::Clicked)                          => chooseRepositoryFolderButton.handle(source, &event),
        (S::ChooseRepositoryFolderDialog,       E::DialogResponded(_))               => chooseRepositoryFolderButton.handle(source, &event),
        (S::CommitDiffViewWidget,               E::ZoomRequested(_))                 => commitDiffView.handle(source, &event),
        (S::CommitAuthorFilterEntry,            E::CommitAuthorFilterChanged(_))     => commitLogModelFilter.handle(source, &event),
        (S::CommitLog,                          E::CommitLogChanged)                 => commitLogModel.handle(source, &event),
        (S::CommitLogModelFilter,               E::MarkCommitForReportToggled(_))    => commitLogModel.handle(source, &event),
        (S::CommitLogView,                      E::CommitSelected(_))                => commitDiffView.handle(source, &event),
        (S::CommitLogView,                      E::CommitUnselected)                 => commitDiffView.handle(source, &event),
        (S::CommitLogViewCheckButton,           E::MarkCommitForReportToggled(_))    => commitLogModelFilter.handle(source, &event),
        (S::CommitLogViewWidget,                E::SelectionChanged(_))              => commitLogView.handle(source, &event),
        (S::GenerateReportButton,               E::GenerateReportRequested)          => reportGenerator.handle(source, &event),
        (S::MonthComboBox,                      E::MonthFilterChanged(_))            => (&mut commitLogModelFilter, &mut outputPathStore).handle(source, &event),
        (S::OpenOptionsButton,                  E::OpenOptionsRequested)             => optionsDialog.handle(source, &event),
        (S::OptionsDialog,                      E::OutputFileNamesPatternChanged(_)) => reportGenerator.handle(source, &event),
        (S::OptionsDialogWidget,                E::DialogResponded(_))               => optionsDialog.handle(source, &event),
        (S::OutputPathStore,                    E::OutputPathChanged(_))             => (&mut outputPathLabel, &mut reportGenerator, &mut configStore).handle(source, &event),
        (S::OutputPathStore,                    E::PartialOutputPathChanged(_))      => outputPathLabel.handle(source, &event),
        (S::RepositoryStore,                    E::RepositoryChanged(_))             => (&mut repositoryPathLabel, &mut commitLog, &mut commitDiffView, &mut reportGenerator, &mut configStore).handle(source, &event),
        (S::YearSpinButton,                     E::YearFilterChanged(_))             => (&mut commitLogModelFilter, &mut outputPathStore).handle(source, &event),
        (source, event) => onUnknown(source, &event) }

        glib::Continue(true)
    });
}

pub struct EventHandlers
{
    pub chooseOutputFolderButton: ChooseFolderButton,
    pub chooseRepositoryFolderButton: ChooseFolderButton,
    pub commitDiffView: CommitDiffView,
    pub commitLog: Rc<RefCell<CommitLog>>,
    pub commitLogFilterModel: CommitLogModelFilter,
    pub commitLogModel: CommitLogModel,
    pub commitLogView: CommitLogView,
    pub configStore: ConfigStore,
    pub optionsDialog: OptionsDialog,
    pub outputPathLabel: OutputPathLabel,
    pub outputPathStore: OutputPathStore,
    pub reportGenerator: ReportGenerator,
    pub repositoryStore: RepositoryStore,
    pub repositoryPathLabel: RepositoryPathLabel
}

impl<T0, T1> EventHandler for (T0, T1)
    where T0: EventHandler, T1: EventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.0.handle(source, event);
        self.1.handle(source, event);
    }
}

impl<T0, T1, T2> EventHandler for (T0, T1, T2)
    where T0: EventHandler, T1: EventHandler, T2: EventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.0.handle(source, event);
        self.1.handle(source, event);
        self.2.handle(source, event);
    }
}

impl<T0, T1, T2, T3> EventHandler for (T0, T1, T2, T3)
    where T0: EventHandler, T1: EventHandler, T2: EventHandler, T3: EventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.0.handle(source, event);
        self.1.handle(source, event);
        self.2.handle(source, event);
        self.3.handle(source, event);
    }
}

impl<T0, T1, T2, T3, T4> EventHandler for (T0, T1, T2, T3, T4)
    where T0: EventHandler, T1: EventHandler, T2: EventHandler, T3: EventHandler, T4: EventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.0.handle(source, event);
        self.1.handle(source, event);
        self.2.handle(source, event);
        self.3.handle(source, event);
        self.4.handle(source, event);
    }
}

impl<T> EventHandler for &mut T
    where T: EventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        (*self).handle(source, event);
    }
}

impl<T> EventHandler for Rc<RefCell<T>>
    where T: EventHandler
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        self.borrow_mut().handle(source, event);
    }
}
