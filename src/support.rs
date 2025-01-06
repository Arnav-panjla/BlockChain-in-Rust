
pub struct Block<Header, Extrinsic> {
    pub header: Header, 
    pub extrinsics: Vec<Extrinsic>,
}

pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

pub type DispatchResult = Result<(), &'static str>;

pub trait Dispatch<Caller, Call> {
    fn dispatch(&self, extrinsic: Extrinsic<Caller, Call>) -> DispatchResult;
}