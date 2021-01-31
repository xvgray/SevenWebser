#![feature(decl_macro)]

mod web;
mod read_prefs;
mod dbus;
mod tools;

use sysinfo::{ProcessExt, System, SystemExt, ProcessorExt};
use std::ffi::CString;
use std::os::raw::c_char;
use crate::web::plot::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
//use std::ops::Deref;
//use std::collections::VecDeque;
use serde_json::{*};
use std::fs::File;
use std::io::Write;
//use std::convert::TryInto;
//use rocket::http::ext::IntoCollection;
use read_prefs::*;
use crate::web::dane_systemu::dane_systemu_to_json;
use crate::web::struktura_chat::{MessageChat};
//use rocket::http::ext::IntoCollection;
use chrono::Local;


//use crate::web::struktura_web::*;

pub static mut DANE_WWW: web::struktura_web::Dane = web::struktura_web::Dane { port: 0, pid: 0, ile_graczy: 0, zajecie_pamieci: 0, obciazenie_serwera: 0_f32, zmienna1: String::new() };

lazy_static! {
    static ref S : Mutex<System> = Mutex::new(System::new_all()); //dane systemu
    static ref CPU_STATS : Mutex<Vec<u64>> = Mutex::new(vec![0_u64; 60]);
    static ref RAM_STATS : Mutex<Vec<u32>> = Mutex::new(vec![0_u32; 360]);
    static ref GAME_CHAT: Mutex<Vec<MessageChat>> = Mutex::new(vec![MessageChat::new()]);
}

#[no_mangle]
pub extern fn get_return() -> *const c_char {
    unsafe {
        DANE_WWW.port = pobierz_port();
    }
    web::funkcje::start_main();
    /*
    Tworzymy plik json z podstawowymi danymi systemu (cpu, mem, itd)
     */
    let _system = System::new_all();
    let _processors = _system.get_processors();
    let cores = num_cpus::get_physical();//_processors.iter().count() as u32;
    let cores_logical = num_cpus::get();
    let _cpu = format!("{}<br/>{}</br/>{} cores ({} logical cores)",
                       _system.get_global_processor_info().get_vendor_id(),
                       _system.get_global_processor_info().get_brand(),
                       cores,
                       cores_logical
    );
    let mut _swap = String::from("Disabled");
    if _system.get_total_swap() != 0 {
        _swap = format!("{} MB", _system.get_total_swap() / 1024);
    }
    let _ram = format!("RAM: {} MB<br/>Swap: {}",
                       _system.get_total_memory() / 1024,
                       _swap
    );
    let _os_info = os_info::get();
    let _os = format!("{}",
                      _os_info.to_string()
    );
    dane_systemu_to_json(_cpu.as_str(), _ram.as_str(), _os.as_str());

    let zwrotka = "Ready";
    let wyjscie = CString::new(zwrotka).unwrap();
    let wyj = wyjscie.as_ptr();
    std::mem::forget(wyjscie);
    wyj
}

/*
    zmienna systemowa LINE_CHAT_7D:
    format: "nazwa_uzytkownika \n wiadomość"
 */
#[no_mangle]
pub extern fn get_chat_line() {
    let wiadomosc_raw= std::env::var("LINE_CHAT_7D").unwrap();
    let dane: Vec<String> = wiadomosc_raw.split('\n').map(String::from).collect();
    let data = Local::now();
    let data_formatted = format!("{}",data.format("%Y-%m-%d %H:%M:%S"));
    let etype : Box<str> = dane[0].clone().into_boxed_str();
    let user = dane[1].clone().into_boxed_str();
    let msg = dane[2].clone().into_boxed_str();

    let mut wiadomosc = MessageChat::new();
    wiadomosc.push_message(String::from(etype), data_formatted, String::from(user), String::from(msg));
    //wiadomosc.push_message(String::from("tada"), String::from("rada"));
    let mut tablica_chatu = GAME_CHAT.lock().unwrap();
    tablica_chatu.push(wiadomosc);
    if tablica_chatu.capacity() > 1001 { //TODO jesli chat > 1000 linijek, usun najstarsza
        tablica_chatu.remove(0);
    }
}

#[no_mangle]
pub extern fn get_data() {
    let zmienna_srodowiskowa = std::env::var("PARAMS7D").unwrap();
    unsafe {
        //let mut s : Option<System> = None; //zmienna s tylko zadeklarowana
        S.lock().unwrap().refresh_process(std::process::id() as i32);
        S.lock().unwrap().refresh_cpu();
        S.lock().unwrap().refresh_memory();
        if let Some(proces) = S.lock().unwrap().get_process(std::process::id() as i32) {

                let obc_proc = (proces.cpu_usage().clone() as u32) / num_cpus::get() as u32;
                //DANE_WWW.obciazenie_serwera = proces.cpu_usage();
                CPU_STATS.lock().unwrap().remove(0);
                CPU_STATS.lock().unwrap().push(((proces.cpu_usage() as u32) / num_cpus::get() as u32) as u64);
                //RAM usage
                DANE_WWW.zajecie_pamieci = proces.memory();
                RAM_STATS.lock().unwrap().remove(0);
                RAM_STATS.lock().unwrap().push((&DANE_WWW.zajecie_pamieci / 1024) as u32);


                //tworzymy/uaktualniamy plik data_sec.json
                let data_sec = json!({
                        "zajecie_pamieci": ((&DANE_WWW.zajecie_pamieci/1024) as u64).to_string(), //MB
                        "obciazenie_procesora": obc_proc.to_string(),
                        //"age": age_last_year + 1,
                });
                // Serialize it to a JSON string.
                let serialize_data_sec = serde_json::to_string(&data_sec).unwrap();
                let mut plik_json_data_sec = File::create("Mods/SevenWebser/scripts/data_sec.json").expect("Unable to create file");
                plik_json_data_sec.write_all(serialize_data_sec.as_bytes()).expect("Unable to write data");

        }
    }

    unsafe {
        DANE_WWW.zmienna1 = zmienna_srodowiskowa;

        rysuj_ram(640, 240, &RAM_STATS.lock().unwrap(), "RAM Usage (MB)", "Mods/SevenWebser/graphics/wykres_ram.svg");
        rysuj_cpu(640, 240, &CPU_STATS.lock().unwrap(), "CPU Load (%)", "Mods/SevenWebser/graphics/wykres.svg");

        //processor_info = S.lock().unwrap().get_global_processor_info().into_collection();
    }
    //send_table(dane_www);
}



/*fn send_table(dane: web::struktura_web::Dane) -> web::struktura_web::Dane{

}*/
