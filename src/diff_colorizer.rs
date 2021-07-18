use crate::line_number::LineNumber;
use crate::text_view::TextView;

use gtk::prelude::TextTagExt as _;


pub struct DiffColorizer
{
    addedLineTag: gtk::TextTag,
    removedLineTag: gtk::TextTag,
    hunkHeaderTag: gtk::TextTag,
    fileHeaderTag: gtk::TextTag,
    tagStartLine: LineNumber,
    state: State
}

enum State
{
    Normal,
    Added,
    Removed,
    FileHeader
}

impl DiffColorizer
{
    pub fn new() -> Self
    {
        let addedLineTag = makeTag("green");
        let removedLineTag = makeTag("red");
        let hunkHeaderTag = makeTag("silver");
        let fileHeaderTag = makeTag("dodgerblue");
        Self{
            addedLineTag,
            removedLineTag,
            hunkHeaderTag,
            fileHeaderTag,
            tagStartLine: 0.into(),
            state: State::Normal}
    }

    pub fn setupTextView(&self, textView: &TextView)
    {
        textView.registerTags(&[&self.addedLineTag, &self.removedLineTag, &self.hunkHeaderTag, &self.fileHeaderTag]);
    }

    pub fn colorize(&mut self, textView: &TextView, diff: &str)
    {
        textView.setText(diff);
        self.applyTags(textView, diff);
    }


    // private

    fn applyTags(&mut self, textView: &TextView, text: &str)
    {
        self.state = State::Normal;
        self.tagStartLine = 0.into();
        self.applyTagsBasedOnLineTypes(textView, text);
        self.closeLastOpenTag(textView);
    }

    fn applyTagsBasedOnLineTypes(&mut self, textView: &TextView, text: &str)
    {
        for (lineNumber, line) in text.lines().enumerate() {
            if let Some(character) = line.chars().next() {
                let lineNumber: LineNumber = lineNumber.into();
                match character {
                    '+' => self.applyTagToAddedLine(textView, lineNumber),
                    '-' => self.applyTagToRemovedLine(textView, lineNumber),
                    '@' => self.applyTagToHunkHeader(textView, lineNumber),
                    ' ' | 'C' | 'A' | 'D' => self.applyTagToNormalLine(textView, lineNumber), // space, Commit, Author, Date
                     _  => self.applyTagToFileHeader(textView, lineNumber),
                }
            }
        }
    }

    fn applyTagToAddedLine(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => {
                self.tagStartLine = lineNumber;
                self.state = State::Added;
            },
            State::Added => (),
            State::Removed => {
                textView.applyTag(&self.removedLineTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Added;
            },
            State::FileHeader => {
                textView.applyTag(&self.fileHeaderTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Added;
            }
        }
    }

    fn applyTagToRemovedLine(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => {
                self.tagStartLine = lineNumber;
                self.state = State::Removed;
            },
            State::Added => {
                textView.applyTag(&self.addedLineTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Removed;
            }
            State::Removed => (),
            State::FileHeader => {
                textView.applyTag(&self.fileHeaderTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Removed;
            }
        }
    }

    fn applyTagToHunkHeader(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => (),
            State::Added => {
                textView.applyTag(&self.addedLineTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            }
            State::Removed => {
                textView.applyTag(&self.removedLineTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            },
            State::FileHeader => {
                textView.applyTag(&self.fileHeaderTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Normal;
            }
        }
        // hunk headers are in the form of "@@ -24,12 +25,14 @@ struct Foo"
        textView.applyTagUntilMatchEnd(&self.hunkHeaderTag, lineNumber, " @@");
    }

    fn applyTagToFileHeader(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => {
                self.tagStartLine = lineNumber;
                self.state = State::FileHeader;
            },
            State::Added => {
                textView.applyTag(&self.addedLineTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::FileHeader;
            }
            State::Removed => {
                textView.applyTag(&self.removedLineTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::FileHeader;
            },
            State::FileHeader => ()
        }
    }

    fn applyTagToNormalLine(&mut self, textView: &TextView, lineNumber: LineNumber)
    {
        match self.state {
            State::Normal => (),
            State::Added => {
                textView.applyTag(&self.addedLineTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            },
            State::Removed => {
                textView.applyTag(&self.removedLineTag, self.tagStartLine, lineNumber);
                self.state = State::Normal;
            },
            State::FileHeader => {
                textView.applyTag(&self.fileHeaderTag, self.tagStartLine, lineNumber);
                self.tagStartLine = lineNumber;
                self.state = State::Normal;
            }
        }
    }

    fn closeLastOpenTag(&self, textView: &TextView)
    {
        match self.state {
            State::Normal => (),
            State::Added => textView.applyTagUntilEnd(&self.addedLineTag, self.tagStartLine),
            State::Removed => textView.applyTagUntilEnd(&self.removedLineTag, self.tagStartLine),
            State::FileHeader => textView.applyTagUntilEnd(&self.fileHeaderTag, self.tagStartLine)
        }
    }
}

fn makeTag(name: &str) -> gtk::TextTag
{
    let tag = gtk::TextTag::new(Some(name));
    tag.set_foreground(Some(name));
    tag
}
