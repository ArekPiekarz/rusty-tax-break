pub enum CommitLogColumn
{
    Report,
    Message,
    Date,
    Author,
    Email,
    OriginalRow
}

impl CommitLogColumn
{
    pub fn allAsArrayOfU32() -> &'static [u32]
    {
        &[0, 1, 2, 3, 4, 5]
    }
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
