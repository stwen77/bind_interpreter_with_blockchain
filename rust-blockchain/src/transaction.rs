#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct transaction {
    from: String,
    to  : String,
    value:u32,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct transaction_module {
    current : Vec<transaction>,
}

impl transaction_module {
    pub fn new()->Self {
        transaction_module {
            current: vec![],
        }
    }

    pub fn create_and_broadcast_transaction(&mut self, from:String, to:String, value: u32) ->Result<(),()>{
        let transac = transaction::new(from,to,value);

        //todo verify transaction

        self.current.push(transac);

        //todo broadcast 

        Ok(())
    }

    pub fn receive_transaction(&mut self, transac :&transaction ) {
        self.current.push(transac.clone());
    }

    pub fn list_transaction_local(&self){
        println!("list_transaction_local:");
        for x in self.current.iter(){
            println!("{:?}", x);
        }
    }
    pub fn get_current(&self)-> &Vec<transaction>{
        &self.current
    }
}
impl transaction {
    fn new(from:String, to:String, value: u32)->Self {
        transaction{
            from :from,
            to   :to,
            value:value,
        }
    }
}