
pub struct transaction {
    from: String,
    to  : String,
    value:i32,
}
pub struct transaction_module {
    current : Vec<transaction>,
}

impl transaction_module {
    fn new()->Self {
        transaction_module {
            current: vec![],
        }
    }
}