use crate::config_store::Config;
use crate::date_time::LocalDate;
use crate::event::{Event, OutputPathInfo, Year};
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::source::Source;

use chrono::Datelike as _;
use std::path::Path;
use std::path::PathBuf;


pub struct OutputPathStore
{
    path: Option<PathBuf>,
    pathPrefix: Option<PathBuf>,
    date: LocalDate,
    sender: Sender
}

impl EventHandler for OutputPathStore
{
    fn handle(&mut self, source: Source, event: &Event)
    {
        match event {
            Event::FolderChosen(path)        => self.onFolderChosen(path),
            Event::MonthFilterChanged(month) => self.onMonthChanged(*month),
            Event::YearFilterChanged(year)   => self.onYearChanged(*year),
            _ => onUnknown(source, event)
        }
    }
}

impl OutputPathStore
{
    pub fn new(config: &Config, date: LocalDate, sender: Sender) -> Self
    {
        match &config.outputPathPrefix {
            Some(prefix) => {
                let mut path = prefix.clone();
                path.push(date.year().to_string());
                path.push(format!("{:02}", date.month()));
                Self{path: Some(path), pathPrefix: Some(prefix.into()), date, sender}
            },
            None => {
                Self{path: None, pathPrefix: None, date, sender}
            }
        }
    }

    pub fn getPath(&self) -> &Option<PathBuf>
    {
        &self.path
    }


    // private

    fn onFolderChosen(&mut self, newPathPrefix: &Path)
    {
        if let Some(pathPrefix) = &self.pathPrefix {
            if *pathPrefix == newPathPrefix {
                return;
            }
        }

        self.pathPrefix = Some(newPathPrefix.into());
        self.updatePath();
    }

    fn onMonthChanged(&mut self, newMonth: chrono::Month)
    {
        if self.date.month() == newMonth.number_from_month() {
            return;
        }
        self.date = self.date.with_month(newMonth.number_from_month()).unwrap();
        self.updatePath();
    }

    fn onYearChanged(&mut self, newYear: Year)
    {
        if self.date.year() == newYear {
            return;
        }
        self.date = self.date.with_year(newYear).unwrap();
        self.updatePath();
    }

    fn updatePath(&mut self)
    {
        match &self.pathPrefix {
            Some(pathPrefix) => {
                let mut path = pathPrefix.clone();
                path.push(self.date.year().to_string());
                path.push(format!("{:02}", self.date.month()));
                self.path = Some(path.clone());
                self.sender.send(
                    (Source::OutputPathStore,
                     Event::OutputPathChanged(OutputPathInfo{full: path, prefix: pathPrefix.into()}))).unwrap();
            },
            None => {
                let mut path = PathBuf::from(self.date.year().to_string());
                path.push(format!("{:02}", self.date.month()));
                self.sender.send((Source::OutputPathStore, Event::PartialOutputPathChanged(path))).unwrap();
            }
        }
    }
}
