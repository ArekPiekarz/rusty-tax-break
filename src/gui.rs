use crate::application_window::ApplicationWindow;
use crate::channel::makeChannel;
use crate::choose_output_folder_button::makeChooseOutputFolderButton;
use crate::output_path_label::OutputPathLabel;
use crate::choose_repository_folder_button::makeChooseRepositoryFolderButton;
use crate::commit_author_filter_entry::setupCommitAuthorFilterEntry;
use crate::commit_diff_view::CommitDiffView;
use crate::commit_log::CommitLog;
use crate::commit_log_model::CommitLogModel;
use crate::commit_log_model_filter::CommitLogModelFilter;
use crate::commit_log_view::CommitLogView;
use crate::dispatcher::{EventHandlers, setupDispatching};
use crate::generate_report_button::setupGenerateReportButton;
use crate::gui_element_provider::GuiElementProvider;
use crate::month_filter_combo_box::setupMonthFilterComboBox;
use crate::output_path_store::OutputPathStore;
use crate::report_generator::ReportGenerator;
use crate::repository_path_label::RepositoryPathLabel;
use crate::repository_store::RepositoryStore;
use crate::year_filter_spin_button::setupYearFilterSpinButton;

use std::cell::RefCell;
use std::rc::Rc;


pub struct Gui
{
    applicationWindow: ApplicationWindow
}

impl Gui
{
    pub fn new() -> Self
    {
        gtk::init().unwrap_or_else(|e| panic!("Failed to initialize GTK. Cause: {}", e));
        let (sender, receiver) = makeChannel();
        let guiElementProvider = GuiElementProvider::new(include_str!("main_window.glade"));

        let currentDate = chrono::Local::today();
        let applicationWindow = ApplicationWindow::new(&guiElementProvider);
        let chooseOutputFolderButton = makeChooseOutputFolderButton(&guiElementProvider, sender.clone());
        let chooseRepositoryFolderButton = makeChooseRepositoryFolderButton(&guiElementProvider, sender.clone());
        let commitDiffView = CommitDiffView::new(&guiElementProvider, sender.clone());
        let commitLog = Rc::new(RefCell::new(CommitLog::new(sender.clone())));
        let commitLogFilterModel = CommitLogModelFilter::new(Rc::clone(&commitLog), &guiElementProvider, sender.clone());
        let commitLogModel = CommitLogModel::new(Rc::clone(&commitLog), &guiElementProvider);
        let commitLogView = CommitLogView::new(Rc::clone(&commitLog), &guiElementProvider, sender.clone());
        let outputPathLabel = OutputPathLabel::new(currentDate, &guiElementProvider);
        let outputPathStore = OutputPathStore::new(currentDate, sender.clone());
        let reportGenerator = ReportGenerator::new(Rc::clone(&commitLog));
        let repositoryStore = RepositoryStore::new(sender.clone());
        let repositoryPathLabel = RepositoryPathLabel::new(&guiElementProvider);
        setupGenerateReportButton(&guiElementProvider, sender.clone());
        setupCommitAuthorFilterEntry(&guiElementProvider, sender.clone());
        setupMonthFilterComboBox(&currentDate, &guiElementProvider, sender.clone());
        setupYearFilterSpinButton(&currentDate, &guiElementProvider, sender);

        let eventHandlers = EventHandlers {
            chooseOutputFolderButton,
            chooseRepositoryFolderButton,
            commitDiffView,
            commitLog,
            commitLogFilterModel,
            commitLogModel,
            commitLogView,
            outputPathLabel,
            outputPathStore,
            reportGenerator,
            repositoryStore,
            repositoryPathLabel
        };
        setupDispatching(eventHandlers, receiver);
        Self{applicationWindow}
    }

    pub fn run(&self)
    {
        self.applicationWindow.show();
        gtk::main();
    }
}
