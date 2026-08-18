#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MsgType {
    AiPrompt = 0,
    FunctionCall = 1,
    Info = 2,
    Error = 3,
}

#[derive(Debug)]
pub struct UnknownMsgType(pub u8);

pub struct AngelMsg<'a> {
    pub header: MsgType,
    pub body: &'a [u8],
}

impl<'a> AngelMsg<'a> {
    pub fn new(header: MsgType, body: &'a [u8]) -> Self {
        Self { header, body }
    }
}

impl TryFrom<u8> for MsgType {
    type Error = UnknownMsgType;
    fn try_from(value: u8) -> Result<Self, UnknownMsgType> {
        match value {
            0 => Ok(MsgType::AiPrompt),
            1 => Ok(MsgType::FunctionCall),
            2 => Ok(MsgType::Info),
            3 => Ok(MsgType::Error),
            _ => Err(UnknownMsgType(value)),
        }
    }
}
