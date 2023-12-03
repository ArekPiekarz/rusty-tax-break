use crate::event::Event;
use crate::source::Source;

use gtk::glib;


pub type Sender = glib::Sender<(Source, Event)>;
pub type Receiver = glib::Receiver<(Source, Event)>;

pub trait EventHandler
{
    fn handle(&mut self, source: Source, event: &Event);
}

#[track_caller]
pub fn onUnknown(source: Source, event: &Event)
{
    panic!("Unknown combination of source and event: {:?}, {:?}", source, event)
}
