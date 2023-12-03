use crate::event::Event;
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::gui_element_provider::GuiElementProvider;
use crate::line_number::LineNumber;
use crate::source::Source;

use gtk::gdk;
use gtk::glib;
use gtk::pango;
use gtk::prelude::CssProviderExt as _;
use gtk::prelude::StyleContextExt as _;
use gtk::prelude::TextBufferExt as _;
use gtk::prelude::TextTagTableExt as _;
use gtk::prelude::TextViewExt as _;
use gtk::prelude::WidgetExt as _;
use std::cmp::{min, max};

const NO_SEARCH_LIMIT: Option<&gtk::TextIter> = None;
const SEARCH_VISIBLE_TEXT: gtk::TextSearchFlags = gtk::TextSearchFlags::from_bits_truncate(
    gtk::TextSearchFlags::VISIBLE_ONLY.bits() | gtk::TextSearchFlags::TEXT_ONLY.bits());


pub struct TextView
{
    buffer: gtk::TextBuffer,
    sender: Sender,
    source: Source,
    style: Style
}

impl EventHandler for TextView
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::ZoomRequested(scrollEvent) => self.onZoomRequested(scrollEvent),
            _ => onUnknown(source, event)
        }
    }
}

impl TextView
{
    pub fn new(
        guiElementProvider: &GuiElementProvider,
        name: &str,
        sender: Sender,
        source: Source)
        -> Self
    {
        let widget = guiElementProvider.get::<gtk::TextView>(name);
        let newSelf = Self{
            buffer: widget.buffer().unwrap(),
            sender,
            source,
            style: Style::new(&widget),
        };

        newSelf.connectWidget(&widget);
        newSelf
    }

    pub fn setText(&self, text: &str)
    {
        self.buffer.set_text(text);
    }

    pub fn clear(&self)
    {
        self.setText("");
    }

    pub fn registerTags(&self, tags: &[&gtk::TextTag])
    {
        let tagTable = self.buffer.tag_table().unwrap();
        for tag in tags {
            assert!(tagTable.add(*tag));
        }
    }

    pub fn applyTag(&self, tag: &gtk::TextTag, startLine: LineNumber, endLine: LineNumber)
    {
        self.buffer.apply_tag(
            tag,
            &self.buffer.iter_at_line(startLine.into()),
            &self.buffer.iter_at_line(endLine.into()));
    }

    pub fn applyTagUntilEnd(&self, tag: &gtk::TextTag, startLine: LineNumber)
    {
        self.buffer.apply_tag(tag, &self.buffer.iter_at_line(startLine.into()), &self.buffer.end_iter());
    }

    pub fn applyTagUntilMatchEnd(&self, tag: &gtk::TextTag, startLine: LineNumber, pattern: &str)
    {
        let startIter = self.buffer.iter_at_line(startLine.into());
        let endIter = startIter.forward_search(pattern, SEARCH_VISIBLE_TEXT, NO_SEARCH_LIMIT).unwrap().1;
        self.buffer.apply_tag(tag, &startIter, &endIter);
    }


    // private

    fn connectWidget(&self, widget: &gtk::TextView)
    {
        let sender = self.sender.clone();
        let source = self.source;
        widget.connect_scroll_event(move |_widget, event| {
            onScrolled(event, &sender, source)
        });
    }

    fn onZoomRequested(&mut self, event: &gdk::EventScroll)
    {
        let newFontSize = self.calculateNewFontSize(event);
        if self.style.font.size == newFontSize {
            return;
        }

        self.loadCss(newFontSize, event);
    }

    fn loadCss(&mut self, newFontSize: FontSize, event: &gdk::EventScroll)
    {
        match self.style.cssProvider.load_from_data(self.formatCss(newFontSize).as_bytes()) {
            Ok(_) => self.style.font.size = newFontSize,
            Err(error) => {
                validateCssError(&error);
                self.style.font.maxSize = Some(self.style.font.size);
                self.reloadCorrectCss(event);
            }
        }
    }

    fn reloadCorrectCss(&mut self, event: &gdk::EventScroll)
    {
        let newFontSize = self.calculateNewFontSize(event);
        match self.style.cssProvider.load_from_data(self.formatCss(newFontSize).as_bytes()) {
            Ok(_) => self.style.font.size = newFontSize,
            Err(e) => panic!("Unexpected error when reloading a corrected CSS: {}", e)
        }
    }

    fn calculateNewFontSize(&self, event: &gdk::EventScroll) -> FontSize
    {
        match getY(event.delta()) {
            y if y < 0.0 => self.calculateHigherFontSize(),
            y if y > 0.0 => self.calculateLowerFontSize(),
            _ => self.style.font.size
        }
    }

    fn calculateHigherFontSize(&self) -> FontSize
    {
        match self.style.font.maxSize {
            Some(maxSize) => min(self.style.font.size + 1, maxSize),
            None => self.style.font.size + 1
        }
    }

    fn calculateLowerFontSize(&self) -> FontSize
    {
        max(self.style.font.size - 1, 1)
    }

    fn formatCss(&self, fontSize: FontSize) -> String
    {
        format!("textview {{font: {}pt {:?}}}", fontSize, self.style.font.family)
    }
}

fn onScrolled(event: &gdk::EventScroll, sender: &Sender, source: Source) -> glib::Propagation
{
    if !event.state().contains(gdk::ModifierType::CONTROL_MASK) {
        return glib::Propagation::Proceed;
    }

    sender.send((source, Event::ZoomRequested(event.clone()))).unwrap();
    glib::Propagation::Stop
}

struct Style
{
    cssProvider: gtk::CssProvider,
    font: Font
}

impl Style
{
    fn new<T>(widget: &T) -> Self
        where T: glib::IsA<gtk::Widget>
    {
        let cssProvider = gtk::CssProvider::new();
        widget.style_context().add_provider(&cssProvider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        Self{
            cssProvider,
            font: Font::new(widget)
        }
    }
}

struct Font
{
    size: FontSize,
    maxSize: Option<FontSize>,
    family: FontFamily
}

type FontSize = i32;
type FontFamily = String;

impl Font
{
    fn new<T>(widget: &T) -> Self
        where T: glib::IsA<gtk::Widget>
    {
        let fontDescription = getFontDescription(widget);
        Self{
            size: getFontSize(&fontDescription),
            maxSize: None,
            family: getFontFamily(&fontDescription)
        }
    }
}

fn getFontDescription<T>(widget: &T) -> pango::FontDescription
    where T: glib::IsA<gtk::Widget>
{
    widget.pango_context().font_description().unwrap()
}

fn getFontSize(fontDescription: &pango::FontDescription) -> FontSize
{
    fontDescription.size() / pango::SCALE
}

fn getFontFamily(fontDescription: &pango::FontDescription) -> FontFamily
{
    fontDescription.family().unwrap().into()
}

const fn getY(coordinates: (f64, f64)) -> f64
{
    coordinates.1
}

fn validateCssError(error: &glib::Error)
{
    match error.kind::<gtk::CssProviderError>() {
        Some(cssProviderError) => {
            if let gtk::CssProviderError::Syntax = cssProviderError {
                if error.to_string() != "<data>:1:19not a number" {
                    panic!("Unexpected CSS provider error message: {}", error)
                }
            } else {
                panic!("Unexpected CSS provider error kind: {}", error)
            }
        },
        None => panic!("Unexpected CSS error: {}", error) }
}
