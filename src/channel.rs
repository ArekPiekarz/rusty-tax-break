use gtk::glib;

const DEFAULT_CONTEXT : Option<&glib::MainContext> = None;

#[must_use]
pub fn makeChannel<MessageType>() -> (glib::Sender<MessageType>, glib::Receiver<MessageType>)
{
    glib::MainContext::channel(glib::Priority::default())
}

pub fn attach<MessageType, HandlerType>(receiver: glib::Receiver<MessageType>, handler: HandlerType)
    where HandlerType: FnMut(MessageType) -> glib::ControlFlow + 'static
{
    receiver.attach(DEFAULT_CONTEXT, handler);
}
