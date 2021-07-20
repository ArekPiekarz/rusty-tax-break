use crate::application_window::ApplicationWindow;
use crate::channel::makeChannel;
use crate::choose_output_folder_button::makeChooseOutputFolderButton;
use crate::choose_repository_folder_button::makeChooseRepositoryFolderButton;
use crate::commit_author_filter_entry::setupCommitAuthorFilterEntry;
use crate::commit_diff_view::CommitDiffView;
use crate::commit_log::CommitLog;
use crate::commit_log_model::CommitLogModel;
use crate::commit_log_model_filter::CommitLogModelFilter;
use crate::commit_log_view::CommitLogView;
use crate::config_path::ConfigPath;
use crate::config_store::ConfigStore;
use crate::dispatcher::{EventHandlers, setupDispatching};
use crate::generate_report_button::setupGenerateReportButton;
use crate::gui_element_provider::GuiElementProvider;
use crate::month_filter_combo_box::setupMonthFilterComboBox;
use crate::open_options_button::setupOpenOptionsButton;
use crate::options_dialog::OptionsDialog;
use crate::output_path_label::OutputPathLabel;
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
    pub fn new(configPath: &ConfigPath) -> Self
    {
        gtk::init().unwrap_or_else(|e| panic!("Failed to initialize GTK. Cause: {}", e));
        let (sender, receiver) = makeChannel();
        let guiElementProvider = GuiElementProvider::new(include_str!("main_window.glade"));

        let configStore = ConfigStore::new(configPath);
        let config = configStore.getConfig();
        let currentDate = chrono::Local::today();
        let outputFileNamesPattern = "<commit_short_id> <commit_summary>";
        let applicationWindow = ApplicationWindow::new(config, &guiElementProvider, sender.clone());
        let chooseOutputFolderButton = makeChooseOutputFolderButton(&guiElementProvider, sender.clone());
        let chooseRepositoryFolderButton = makeChooseRepositoryFolderButton(&guiElementProvider, sender.clone());
        let optionsDialog = OptionsDialog::new(outputFileNamesPattern, sender.clone());
        let outputPathLabel = OutputPathLabel::new(config, currentDate, &guiElementProvider);
        let outputPathStore = OutputPathStore::new(config, currentDate, sender.clone());
        let repositoryStore = RepositoryStore::new(config, sender.clone());
        let repository = repositoryStore.getRepository();
        let repositoryPathLabel = RepositoryPathLabel::new(repositoryStore.getRepositoryPath(), &guiElementProvider);
        let commitLog = Rc::new(RefCell::new(CommitLog::new(repository, sender.clone())));
        let commitLogFilterModel = CommitLogModelFilter::new(Rc::clone(&commitLog), &guiElementProvider, sender.clone());
        let commitLogModel = CommitLogModel::new(Rc::clone(&commitLog), &guiElementProvider);
        let commitLogView = CommitLogView::new(Rc::clone(&commitLog), &guiElementProvider, sender.clone());
        let commitDiffView = CommitDiffView::new(repository.clone(), &guiElementProvider, sender.clone());
        let reportGenerator = ReportGenerator::new(
            Rc::clone(&commitLog), repository.clone(),  outputPathStore.getPath().clone(), outputFileNamesPattern);
        setupOpenOptionsButton(&guiElementProvider, sender.clone());
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
            configStore,
            optionsDialog,
            outputPathLabel,
            outputPathStore,
            reportGenerator,
            repositoryStore,
            repositoryPathLabel
        };
        setupDispatching(eventHandlers, receiver);
        Self{applicationWindow}
    }

    pub fn show(&self)
    {
        self.applicationWindow.show();
    }

    pub fn run(&self)
    {
        gtk::main();
    }
}
