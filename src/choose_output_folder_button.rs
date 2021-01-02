use crate::event_handling::Sender;
use crate::gui_element_provider::GuiElementProvider;
use crate::choose_folder_button::ChooseFolderButton;
use crate::source::Source;

pub fn makeChooseOutputFolderButton(guiElementProvider: &GuiElementProvider, sender: Sender) -> ChooseFolderButton
{
    ChooseFolderButton::new(
        "chooseOutputFolderButton",
        "Choose output folder",
        Source::ChooseOutputFolderButton,
        Source::ChooseOutputFolderButtonWidget,
        Source::ChooseOutputFolderDialog,
        guiElementProvider,
        sender
    )
}
