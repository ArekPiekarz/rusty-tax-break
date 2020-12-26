use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::choose_folder_button::ChooseFolderButton;
use crate::source::Source;

pub fn makeChooseRepositoryFolderButton(guiElementProvider: &GuiElementProvider, sender: Sender) -> ChooseFolderButton
{
    ChooseFolderButton::new(
        "chooseRepositoryFolderButton",
        "Choose repository folder",
        Source::ChooseRepositoryFolderButton,
        Source::ChooseRepositoryFolderButtonWidget,
        guiElementProvider,
        sender
    )
}
