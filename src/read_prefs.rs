use serde::*;
use serde_xml_rs::*;
use std::fs::File;
use std::io::Read;

//use std::path::PathBuf;
//use serde::de::value::UsizeDeserializer;
//use std::num::ParseIntError;

//--------------- serverconfig
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ServerSettings {
    pub property: Vec<Property>,
}

/*
------------------ serveradmin.xml
*/
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    #[allow(non_snake_case)]
    pub steamID: String,
    pub permission_level: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Admin {
    pub user: Vec<User>,
    //pub admin: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AdminTools {
    pub admins: Vec<Admin>,
    //pub permissions: Vec<permission>,
    //pub blacklist: Vec<blacklisted>,
}

pub fn pobierz_port() -> u16 {
    //set_logger();
    let mut port = 8080u16;
    let settingsy = pobierz_ustawienia().property;
    //let settingsy = wczytaj_ustawienia_xml("serverconfig.xml").property;
    for p in settingsy {
        if p.name.to_ascii_lowercase() == "telnetport" {
            port = p.value.parse::<u16>().unwrap() + 2;
        }
    }
    log::info!("Setting Port to: {}", &port);
    port
}

pub fn pobierz_nazwe() -> Box<String> {
    //set_logger();
    let mut nazwa = String::from("Server 7dtd");
    let settingsy = pobierz_ustawienia().property;
    //let settingsy = wczytaj_ustawienia_xml("serverconfig.xml").property;
    for p in settingsy {
        if p.name.to_ascii_lowercase() == "servername" {
            nazwa = p.value;
        }
    }
    log::info!("Setting Web Name to: {}", &nazwa);
    Box::new(nazwa)
}

pub fn pobierz_ustawienia() -> Box<ServerSettings> {
    let settingsy = wczytaj_ustawienia_xml("serverconfig.xml");
    Box::new(settingsy)
}

fn znajdz_sciezke_serveradmins() -> String {
    let mut sciezka = String::new();
//TODO zle pobiera sciezke z pliku xml!
    let settingsy = pobierz_ustawienia().property;
    for p in settingsy {
        if p.name.to_ascii_lowercase() == "savegamefolder" {
            sciezka.push_str(p.value.as_str()); //jesli istnieje to podmien sciezke na wpisana przez uzytkownika
        }
    }
    if sciezka.is_empty() {
        match dirs::home_dir() {
            None => {}
            Some(patch) => {
                sciezka.push_str(&patch.to_str().unwrap());
                sciezka.push_str("/.local/share/7DaysToDie/Saves");
            }
        }
    }
    sciezka.push_str("/serveradmin.xml"); //dodaj plik do wczytania na koncu sciezki
    sciezka
}

/*fn znajdz_sciezke_serveradmins() -> Box<String> {
    let mut sciezka = String::new();
    match dirs::home_dir() {
        None => {
            log::warn!("Error...");
        }
        Some(patch) => {
            sciezka.push_str(&patch.to_str().unwrap());
            sciezka.push_str("/.local/share/7DaysToDie/Saves");
        }
    } //zapisano domyslna sciezke: [pobrany_katalog_domowy]/.local/share/7DaysToDie/Saves
    //na wszelki wypadek sprawdz czy istnieje wlasciwosc "SaveGameFolder"

    /*let settingsy = pobierz_ustawienia().property;
    for p in settingsy {
        if p.name.to_ascii_lowercase() == "savegamefolder" {
            sciezka.clear();
            sciezka.push_str(p.value.as_str()); //jesli istnieje to podmien sciezke na wpisana przez uzytkownika
        }
    }*/
    sciezka.push_str("/serveradmin.xml"); //dodaj plik do wczytania na koncu sciezki
    log::info!("serveradmins.xml path: {}", &sciezka);
    Box::new(sciezka)
}*/

fn wczytaj_serveradmins_xml() -> Option<AdminTools> {
    let sciezka = znajdz_sciezke_serveradmins();
    //let sciezka = "/home/marcin/.local/share/7DaysToDie/Saves/serveradmin.xml".to_string();

    let mut bufor = String::new();
    let plik_ustawien = File::open(sciezka);

    //log::info!("path: {}",&sciezka);
    match plik_ustawien
    {
        Ok(mut file) => {
            let result = file.read_to_string(&mut bufor);
            match result {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Err(_) => {
            log::warn!("Błąd wczytania pliku serveradmin.xml");
            panic!("Can't read from serveradmin.xml file");
        }
    };
    //Wywalić BOM z początku stringa
    let bomless = bufor.trim_start_matches("\u{FEFF}");
    let serialize_ustawienia: Result<AdminTools, Error> = serde_xml_rs::from_str(bomless);
    match serialize_ustawienia {
        Ok(wynik) => {
            Some(wynik)
        }
        Err(_) => {
            log::warn!("Error parsing in serveradmin.xml file");
            None
        }
    }
}

pub fn admins_list() -> Option<Vec<String>> {
    let serveradmins = wczytaj_serveradmins_xml();
    let mut wyjscie: Vec<String> = Vec::new();
    match serveradmins {
        None => {
            None
        }
        Some(plik) => {
            let settingsy = plik.admins;
            for adminek in settingsy {
                let userek = adminek.user;
                for admin_nr in userek {
                    let level = admin_nr.permission_level.parse::<i32>();
                    match level {
                        Ok(liczba) => {
                            if liczba == 0 {
                                wyjscie.push(admin_nr.steamID);
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Some(wyjscie)
        }
    }
}

fn wczytaj_ustawienia_xml(sciezka: &str) -> ServerSettings {
    let mut bufor = String::new();
    let plik_ustawien = File::open(sciezka);
    //log::info!("ścieżka: {}",sciezka);
    match plik_ustawien
    {
        Ok(mut file) => {
            let result = file.read_to_string(&mut bufor);
            match result {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Err(_) => {
            log::warn!("Błąd wczytania pliku ustawień");
            panic!("Can't read from settings file");
        }
    };
    let serialize_ustawienia = serde_xml_rs::from_str(bufor.as_str()).unwrap();
    serialize_ustawienia
}


//ustawienie logowania
/*fn set_logger() {
    let result = simple_logging::log_to_file("Mods/SevenWebser/output.log", LevelFilter::Info);
    match result {
        Ok(_) => {}
        Err(_) => {}
    }
}*/
