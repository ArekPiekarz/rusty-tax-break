pub enum CommitLogColumn
{
    Report,
    Message,
    Date,
    Author,
    Email,
    OriginalRow
}

impl From<CommitLogColumn> for i32
{
    fn from(value: CommitLogColumn) -> Self
    {
        value as Self
    }
}

impl From<CommitLogColumn> for u32
{
    fn from(value: CommitLogColumn) -> Self
    {
        value as Self
    }
}

pub type OriginalRow = u32;
