pub struct Dane {
    pub port: u16,
    pub pid: u32,
    pub ile_graczy: i32,
    pub zajecie_pamieci: u64,
    pub obciazenie_serwera: f32,
    pub zmienna1: String,
}

impl Dane{
    pub fn new() -> Dane{
        //let tablica_mem : Vec<u64> = vec![0u64; 10]; //20 wyzerowanych komórek

        Dane {
            port: 0,
            pid: std::process::id(), //nr pid
            ile_graczy: 0,
            zajecie_pamieci: 0,
            obciazenie_serwera: 0_f32,
            zmienna1: String::new(),
        }

    }

    pub fn fill(&mut self){
        //self.pid = std::process::id();
        //self.zmienna1 = String::from("nie pobralo :(");
    }
}