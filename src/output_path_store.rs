use crate::date_time::LocalDate;
use crate::event::{Event, FolderUriStr, Year};
use crate::event_handling::{EventHandler, onUnknown, Sender};
use crate::source::Source;

use chrono::Datelike as _;
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
            Event::FolderChosen(folderUri)   => self.onFolderChosen(folderUri),
            Event::MonthFilterChanged(month) => self.onMonthChanged(*month),
            Event::YearFilterChanged(year)   => self.onYearChanged(*year),
            _ => onUnknown(source, event)
        }
    }
}

impl OutputPathStore
{
    pub fn new(date: LocalDate, sender: Sender) -> Self
    {
        Self{path: None, pathPrefix: None, date, sender}
    }


    // private

    fn onFolderChosen(&mut self, folderUri: &FolderUriStr)
    {
        match glib::filename_from_uri(folderUri) {
            Ok((path, hostnameOpt)) => {
                match hostnameOpt {
                    Some(hostname) => eprintln!("Hostnames are not supported when choosing outputs: {}", hostname),
                    None => self.onValidFolderChosen(path)
                }
            },
            Err(e) => eprintln!("Getting path from folder URI failed: {}", e)
        }
    }

    fn onValidFolderChosen(&mut self, newPathPrefix: PathBuf)
    {
        if let Some(pathPrefix) = &self.pathPrefix {
            if *pathPrefix == newPathPrefix {
                return;
            }
        }

        self.pathPrefix = Some(newPathPrefix);
        self.updatePath();
    }

    fn onMonthChanged(&mut self, month: chrono::Month)
    {
        self.date = self.date.with_month(month.number_from_month()).unwrap();
        self.updatePath();
    }

    fn onYearChanged(&mut self, year: Year)
    {
        self.date = self.date.with_year(year).unwrap();
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
                self.sender.send((Source::OutputPathStore, Event::OutputPathChanged(path))).unwrap();
            },
            None => {
                let mut path = PathBuf::from(self.date.year().to_string());
                path.push(format!("{:02}", self.date.month()));
                self.sender.send((Source::OutputPathStore, Event::PartialOutputPathChanged(path))).unwrap();
            }
        }
    }
}
