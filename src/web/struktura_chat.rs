pub struct MessageChat {
    pub user: String,
    pub date: String,
    pub etype: String, //Chat type: Global=0, Friends=1, Party=2, Whisper=3
    pub message: String,
}

impl MessageChat {
    pub fn new() -> MessageChat{
        //let tablica_mem : Vec<u64> = vec![0u64; 10]; //20 wyzerowanych komórek
        MessageChat {
            user: String::with_capacity(50),
            date: String::with_capacity(50),
            etype: String::with_capacity(10),
            message: String::with_capacity(500),
        }

    }

    pub fn push_message(&mut self, etype: String, date: String, user: String, msg: String) {

        self.etype = etype;
        self.date = date;
        self.user = user;
        self.message = msg;

    }


}
