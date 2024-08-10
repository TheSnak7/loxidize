pub struct Uninitialized;
pub struct Initialized;

pub trait State {}

impl State for Uninitialized {}

impl State for Initialized {}
