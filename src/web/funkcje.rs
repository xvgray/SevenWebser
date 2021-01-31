use rocket::*;
use rocket::response::content::{Html, Css};
use rocket::config::{Config, Environment};
use rocket_contrib::serve::StaticFiles;
use std::{thread};
use std::fs::File;
//use crate::web;

use crate::{GAME_CHAT};
use crate::{DANE_WWW};
use std::io::{Read, Write};
//use rocket::http::ext::IntoCollection;
//use serde::{Deserialize, Serialize};
use crate::read_prefs::{pobierz_ustawienia, pobierz_nazwe, pobierz_port, admins_list};
use crate::web::funkcje_wypelniania::{_wypelnij_game_params_icons, _wypelnij_chat, _wypelnij_chat_users};
//use std::sync::Mutex;
use rocket::response::{Redirect};
use rocket::http::{Cookie, Cookies, SameSite};
use rocket::request::{FromForm,Form};

//use crate::web::struktura_chat::MessageChat;
use crate::dbus::{dbus_send_chat, dbus_get_online_players};
use log::LevelFilter;
//use dbus::arg::RefArg;
//use std::ops::Deref;
//use std::borrow::Borrow;

#[get("/info")]
fn info() -> Html<String> {
    let mut bufor = String::new();

        //wczytujemy ciało strony w HTML
        bufor.push_str(get_site_part("header").as_str());


        //let _zmienne = DANE_WWW.zmienna1.replace(&['\"', ' '][..], "");
        //let zmienne: Vec<&str> = _zmienne.split("<br>").collect();
        //let parametry: Vec<&str> = zmienne[3].split(':').collect();
        //zrobmy plik json z parametrami gry
        let mut plik_json_data_settings = File::create("Mods/SevenWebser/scripts/data_settings.json").expect("Unable to create file");
        //let mut zawartosc = String::new();


        let ustawienia = *pobierz_ustawienia();
        let serializowane = serde_json::to_string(&ustawienia).unwrap();
        //Zapis danych do pliku json
        //plik_json_data_settings.write_all(serializowane.as_bytes()).expect("Unable to write data");
        plik_json_data_settings.write_all(serializowane.as_bytes()).expect("Unable to write data");
        //plik_json_data_settings.write_all(&zawartosc.as_bytes()).expect("Unable to write data");

        //Główny panel środkowy, na którym są rozmieszczone w kolumnach inne panele
        bufor.push_str(format!(r#"<div id="MAIN_PANEL">"#).as_str());

        /*
        Najpierw panel lewy z info systemowymi (procesor, pamięć, system)
         */
        bufor.push_str(format!(r#"<div id="GAME_PARAMS">"#).as_str());
        bufor.push_str(format!(r#"<div id="PANEL_TITLE">Process Info</div>"#).as_str());
        //bufor.push_str(format!(r#"<div id="PANEL_PARAMS"#).as_str());
        //bufor.push_str(format!("Parametry serwera: {}<br>", zmienne[20]).as_str());
        //bufor.push_str(format!(r#"<img src="graphics/7dtd.png" alt="7dtd" title="sdtd">"#).as_str());
        bufor.push_str(format!(r#"<div id="DYN_CPU"><img id="_DYN_CPU" src="graphics/wykres.svg" alt="fetching data..."></div>"#).as_str());
        bufor.push_str(format!(r#"<div id="DYN_RAM"><img id="_DYN_RAM" src="graphics/wykres_ram.svg" alt="fetching data..."></div>"#).as_str());
        bufor.push_str(format!(r#"<div id="PANEL_TITLE">System Info</div>"#).as_str());
        bufor.push_str(format!(r#"<div id="SERVER_PARAMS"><table id="_SYSTEM_PARAMS"></table></div>"#).as_str());
        //bufor.push_str(format!(r#"<div id="ROZDZIELACZ"> </div>"#).as_str());
        //bufor.push_str(format!(r#"<div id="_DYNAMIC"><img src="graphics/wykres.svg" alt="wykres" title="plot"></div>"#).as_str());
        bufor.push_str(format!(r#"</div>"#).as_str());


        /*
        Panel środkowy z ustawieniami gry
        Ikonki + okienko ze zrzutem danych z config.xml
         */
        bufor.push_str(format!(r#"<div id="GAME_PARAMS">"#).as_str());
        bufor.push_str(format!(r#"<div id="PANEL_TITLE">Server Config</div>"#).as_str());
        bufor.push_str(format!("{}", _wypelnij_game_params_icons()).as_str());
        bufor.push_str(format!(r#"<div id="PANEL_TITLE">File "serverconfig.xml"</div>"#).as_str());
        bufor.push_str(format!(r#"<div id="SERVER_PARAMS"><table id="_GAME_PARAMS"></table></div>"#).as_str());
        bufor.push_str(format!(r#"</div>"#).as_str());

        /*
        Ostatni panel z listą graczy
         */
        bufor.push_str(format!(r#"<div id="GAME_PARAMS">"#).as_str());
        bufor.push_str(format!(r#"<div id="PANEL_TITLE">Server Statistics</div>"#).as_str());
        //bufor.push_str(format!(r#"<div id="SERVER_PARAMS"><table id="_GAME_PARAMS"></table></div>"#).as_str());
        bufor.push_str(format!(r#"</div>"#).as_str());

        //----------------------------------------------------
        //Domknięcie głównego panela
        bufor.push_str(format!(r#"</div>"#).as_str());

        //wstawiamy kody js
        bufor.push_str(format!(r#"<script src="/scripts/dynamic.js"></script>"#).as_str());

        //koniec <body>
        bufor.push_str(format!(r#"</body></html>"#).as_str());



    Html(bufor)
}

//"lista użytkowników, itemów, claimów, materacy. Z podglądem statystyk. Bany, ostrzeżenia, kicki, unbany"
#[get("/users")]              // <- route attribute
fn users() -> Html<String> {  // <- request handler
    let mut bufor = String::new();
    bufor.push_str(get_site_part("header").as_str());

    bufor.push_str(format!(r#"<div id="MAIN_PANEL">"#).as_str());
    bufor.push_str(format!(r#"<div id="GAME_PARAMS">"#).as_str());


    //zamkniecie pliku!
    bufor.push_str(format!(r#"</body></html>"#).as_str());
    Html(bufor)
}

//"interakcja z graczami i podgląd chatu"
#[get("/chat")]              // <- route attribute
fn chat() -> Html<String> {  // <- request handler
    let mut bufor = String::new();
    bufor.push_str(get_site_part("header").as_str());

    bufor.push_str(format!(r#"<div id="MAIN_PANEL">"#).as_str());
    bufor.push_str(format!(r#"<div id="GAME_PARAMS">"#).as_str());

    //funkcje czatu itp
    bufor.push_str(format!(r#"<div id="PANEL_TITLE">Game Chat</div>"#).as_str());

    //let drained_vec: Drain<Message_Chat> = GAME_CHAT.lock().unwrap().drain(..);
    //let mut copy_chat: Vec<Message_Chat> = Vec::new();
    //copy_chat.append(&mut *GAME_CHAT.lock().unwrap()); //dziala ale kasuje zrodlo
    //let temp = *GAME_CHAT.lock().unwrap();
    //let copy_chat: &Vec<MessageChat> = temp.clone();
    //let copy_chat: Vec<MessageChat> = *GAME_CHAT.lock().unwrap();
    //let userzy = dbus_get_online_players().unwrap_or_default();
    let userzy = dbus_get_online_players().unwrap_or_default();

    //okno chatu
    bufor.push_str(format!(r#"<div id="CHAT">"#).as_str());
    bufor.push_str(format!("{}", _wypelnij_chat(&*GAME_CHAT.lock().unwrap())).as_str());
    bufor.push_str(format!("{}", _wypelnij_chat_users(userzy)).as_str());
    bufor.push_str(format!(r#"</div>"#).as_str()); //koniec div CHAT

    /*kontrolki wysylania wiadomosci*/
    bufor.push_str(format!(r##"
            <div id="SEND_CHAT">
            <form id="SendMessage">
                <input type="color" id="colorpicker" value="#FFFFFF"/>
                <input type="text" id="usermsg" />
                <input type="submit" id="submitmsg" value="Send"/>
            </form> </div>
        "##).as_str());

    bufor.push_str(format!(r#"</div></div>"#).as_str());


    /* Plik js */
    bufor.push_str(format!(r#"<script src="/scripts/chat.js"></script>"#).as_str());
    //zamkniecie pliku!
    bufor.push_str(format!(r#"</body></html>"#).as_str());

    Html(bufor)

}


//"Podgląd mapy z możliwością interakcji. Tracking graczy, alerty LCB"
#[get("/map")]              // <- route attribute
fn map() -> Html<String> {  // <- request handler
    let mut bufor = String::new();
    bufor.push_str(get_site_part("header").as_str());
    bufor.push_str(format!(r#"<div id="MAIN_PANEL">"#).as_str());
    bufor.push_str(format!(r#"<div id="GAME_PARAMS">"#).as_str());

//zamkniecie pliku!
    bufor.push_str(format!(r#"</body></html>"#).as_str());
    Html(bufor)
}

//"Zarządzanie serwerem: restarty, komunikaty, resetowanie terenu, telnet po ssh"
#[get("/admin")]              // <- route attribute
fn admin() -> Html<String> {  // <- request handler
    let mut bufor = String::new();
    bufor.push_str(get_site_part("header").as_str());
    bufor.push_str(format!(r#"<div id="MAIN_PANEL">"#).as_str());
    bufor.push_str(format!(r#"<div id="GAME_PARAMS">"#).as_str());

    let admini = admins_list().unwrap_or(vec!["Admins list empty :(".to_string()]);
    for adminek in admini {
        bufor.push_str(format!(r#"<div id="PANEL_TITLE">{}</div>"#, adminek).as_str());
    }
    bufor.push_str(format!(r#"</div></div>"#).as_str());

    //zamkniecie pliku!
    bufor.push_str(format!(r#"</body></html>"#).as_str());
    Html(bufor)
}

#[catch(404)]
fn not_found() -> Html<String> {  // <- request handler
    let mut bufor = String::new();
    bufor.push_str(get_site_part("header").as_str());

    bufor.push_str(format!(r#"<div id="MAIN_PANEL">"#).as_str());
    bufor.push_str(format!(r#"<div id="GAME_PARAMS">"#).as_str());

    bufor.push_str(format!(r#"<div id="PANEL_TITLE">Error 404</div><br>"#).as_str());
    //bufor.push_str(format!(r#"<div id="MENU_LOGIN"><a href="/steamlogin"><img id="steam_img" border="0" alt="Steam Login" src="/graphics/steam_sign.png"></a></div>"#).as_str());

    bufor.push_str(format!(r#"</div></div>"#).as_str());


    Html(bufor)
}

//strona logowania, narzucona odgórnie
#[get("/login")]              // <- route attribute
fn login() -> Html<String> {  // <- request handler
    //Ok(response.body(Vec::new())?)

    let mut bufor = String::new();
    bufor.push_str(get_site_part("header").as_str());

    bufor.push_str(format!(r#"<div id="MAIN_PANEL">"#).as_str());
    bufor.push_str(format!(r#"<div id="GAME_PARAMS">"#).as_str());

    bufor.push_str(format!(r#"<div id="PANEL_TITLE">Please Login</div><br><br><br>"#).as_str());
    bufor.push_str(format!(r#"<div id="MENU_LOGIN"><a href="/steamlogin"><img id="steam_img" border="0" alt="Steam Login" src="/graphics/steam_sign.png"></a></div>"#).as_str());

    bufor.push_str(format!(r#"</div></div>"#).as_str());

    Html(bufor)
}



fn check_if_admin(cookies: &mut Cookies) -> bool {
    let user = cookies.get_private("user");
    let mut czy_uprawniony = false;
    match user {
        None => {
            czy_uprawniony = false;
        }
        Some(cookie_steamid) => {
            for admin_ in admins_list().unwrap_or(vec!["Admins list empty :(".to_string()]) {
                if cookie_steamid.value().contains(admin_.as_str()) {
                    czy_uprawniony = true;
                }
                /*if cookie_steamid.value().contains("76561198032259226") {
                czy_uprawniony = true;
            } else {
                czy_uprawniony = false;
            }*/ //jesli haslo - steamid sie nie zgadza
            }
        }
    }
    czy_uprawniony
    //true
}

#[get("/<site>")]
fn submit(mut cookies: Cookies, site: Option<String>) -> Html<String> {
    if check_if_admin(&mut cookies) {
        match site {
            None => {
                info()
            }
            Some(strona) => {
                match strona.as_str() {
                    "info" => { info() }
                    "users" => { users() }
                    "chat" => { chat() }
                    "map" => { map() }
                    "admin" => { admin() }
                    //"about" => { about() }
                    _ => { //404 error
                        not_found()
                    }
                }
            }
        }
    } else {
        login()
    }
}

/* Odbiorca wiadomości z chatu */
#[get("/message_chat?<user>&<msg>")]
fn send_message(mut cookies: Cookies, user: Option<String>, msg: Option<String>) -> Html<String> {
    if check_if_admin(&mut cookies) {
        message_chat(user, msg)
    } else {
        login()
    }
}
/* Odbieramy wiadomość
    do zapisania na czacie
 */
//#[get("/message_chat")]              // <- route attribute
fn message_chat(user: Option<String>, msg: Option<String>) -> Html<String> {  // <- request handler
    let mut bufor = String::new();
    bufor.push_str(get_site_part("header").as_str());

    let _usr = user.unwrap_or(String::from("")).to_string();
    let _msg = msg.unwrap_or(String::from("")).to_string();

    let result = dbus_send_chat(_usr, _msg);
    match result {
        Ok(_list) => {
            log::info!("DBUS: {}", "Chat sending successfully");
        }
        Err(_) => {
            log::error!("DBUS: {}", "Webser chat problem");
        }
    }

    /*let mut wiadomosc = Message_Chat::new();
    wiadomosc.push_message(String::from("Global"), String::from("000007"), _usr, _msg);
    //wiadomosc.push_message(String::from("tada"), String::from("rada"));
    let mut tablica_chatu = GAME_CHAT.lock().unwrap();
    tablica_chatu.push(wiadomosc);
    */
    //zamkniecie pliku!
    bufor.push_str(format!(r#"</body></html>"#).as_str());
    Html(bufor)
}

//przekierowuje z glownej strony do /info
#[get("/", rank = 2)]
fn main_redirect() -> Redirect {
    Redirect::to("/info")
}

/*  Odbieramy dane ze Steam
    oraz tworzymy cookie ze steamid
 */
#[get("/steam_redirector?<input..>")]              // <- route attribute
fn steam_redirector(mut cookies: Cookies, input: Option<Form<DaneSteam>>) -> Redirect {  // <- request handler
    let user = "user";
    let mut pass = String::new();
    match input {
        None => {
            //Redirect::to("/")
        }
        Some(buff) => {
            //tworzymy prywatne cookie
            pass.push_str(buff.identity.as_str());
            //cookies.add_private(Cookie::new(user, "1500100900"));

            //Redirect::to("/")
        }
    };
    //possible memory leak, be crfly
    let buf: &'static str = Box::leak(pass.into_boxed_str());
    let mut ciacho = Cookie::new(user, buf);
    ciacho.set_same_site(SameSite::Lax);
    cookies.add_private(ciacho);
    Redirect::to("/")
}

/* Dane wiadomości dla chatu */
/*#[derive(FromForm)]
struct DaneWej {
    user: String,
    message: String,
}*/

/* Funkcja logowania Steam */
#[derive(FromForm)]
struct DaneSteam {
    #[allow(unused)]
    #[form(field = "openid.ns")]
    ns: String,
    #[allow(unused)]
    #[form(field = "openid.mode")]
    mode: String,
    #[allow(unused)]
    #[form(field = "openid.op_endpoint")]
    op_endpoint: String,
    #[allow(unused)]
    #[form(field = "openid.claimed_id")]
    claimed_id: String,
    #[allow(unused)]
    #[form(field = "openid.identity")]
    identity: String,
    #[allow(unused)]
    #[form(field = "openid.return_to")]
    return_to: String,
    #[allow(unused)]
    #[form(field = "openid.response_nonce")]
    response_nonce: String,
    #[allow(unused)]
    #[form(field = "openid.assoc_handle")]
    assoc_handle: String,
    #[allow(unused)]
    #[form(field = "openid.signed")]
    signed: String,
    #[allow(unused)]
    #[form(field = "openid.sig")]
    sig: String,
}

#[get("/steamlogin")]
fn steamlogin() -> Redirect {
    //TODO ustawic pobieranie IP lub nazwy serwera z pliku ustawien!
    let mut myip = String::from("0.0.0.0");
    let ip = my_internet_ip::get();
    match ip {
        Ok(ip) => {
            myip.clear();
            myip.push_str(&ip.to_string());
        }
        Err(e) => log::info!("Could not get IP: {:?}", e)
    };
    let redirector = steam_auth::Redirector::new(format!("http://{}:{}", myip, pobierz_port()), "/steam_redirector").unwrap();
    let przekierowanie = redirector.url().to_string();

    let buf: &'static str = Box::leak(przekierowanie.into_boxed_str());
    Redirect::to(buf)
    //("https://steamcommunity.com/oauth/login?response_type=token&client_id=client_id_here&state=whatever_you_want"))
}

//szczątkowe elementy strony (menu, belka dolna)
fn get_site_part(part: &str) -> String {
    let mut wyjscie = String::new();
    match &part[..] {
        "header" => {
            wyjscie.push_str(r#"<!DOCTYPE html><html lang="en"><head>
	        <meta http-equiv="content-type" content="text/html; charset=utf-8">
	        <link href="/styles/style.css" rel="stylesheet" type="text/css">
	        <link rel="icon" type="image/png" href="/graphics/favicon.png"><title>
	        "#);
            let nazwa = *pobierz_nazwe();
            wyjscie.push_str(&*format!(r#"{}</title></head>"#, nazwa));

            //zaczynamy element <body>
            wyjscie.push_str(format!(r#"<body>"#).as_str());

            //wstawiamy js do przeładowywania DIV
            wyjscie.push_str(format!(r#"<script
			  src="https://code.jquery.com/jquery-3.5.1.js"
			  integrity="sha256-QWo7LDvxbWT2tbbQ97B53yJnYU3WhH/C8ycbRAkjPDc="
			  crossorigin="anonymous"></script>"#).as_str());
            wyjscie.push_str(format!(r#"<script src="/scripts/footer.js"></script>"#).as_str());

            //DANE_WWW.pid = std::process::id();
            wyjscie.push_str(format!(r#"
	  <div id="MENU">
		<div id="MENU_POS">
		  <div><a href="/info"><img alt="Info" src="/graphics/icon_info.svg"></a></div>
		  <div><a href="/users"><img alt="Users" src="/graphics/icon_users.svg"></a></div>
		  <div><a href="/chat"><img alt="Chat" src="/graphics/icon_chat.svg"></a></div>
		  <div><a href="/map"><img alt="Map" src="/graphics/icon_map.svg"></a></div>
		  <div><a href="/admin"><img alt="Admin" src="/graphics/icon_admin.svg"></a></div>
		  <div><a href="/about"><img alt="About" src="/graphics/icon_about.svg"></a></div>
		</div>

	  </div>
	  <div id="STOPKA"><table id="footer_table"><tr><td id="footer_left">fetching data...</td><td id="footer_right"><img id="footer_rotor" alt="*" src="/graphics/rotor.png"></td></tr></table></div>"#).as_str());
            //<div id="STOPKA">fetching data...</div>"#).as_str());

            wyjscie
        }
        _ => "ERROR: PART DATA MISMATCH!".to_string(),
    }
}

#[get("/styles/style.css")]
fn style() -> Css<String> {
    //log::info!("Próba wczytania pliku css");
    let mut bufor = String::new();
    let plik_css = File::open("Mods/SevenWebser/styles/style.css");
    match plik_css
    {
        Ok(mut file) => file.read_to_string(&mut bufor).unwrap(),
        Err(_) => {
            log::warn!("Błąd wczytania pliku css");
            panic!("Can't read from file");
        }
    };
    Css(bufor)
}

pub fn start_main() {
    simple_logging::log_to_file("Mods/SevenWebser/output.log", LevelFilter::Info).unwrap_or(());
    unsafe {
        let rocket_config = Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(DANE_WWW.port as u16)
            //.log_level(rocket::logger::LoggingLevel::Critical)
            .finalize().unwrap();
        //let rocket_config2 = rocket_config.clone();

        thread::spawn(|| {
            rocket::custom(rocket_config)
                .mount("/", routes![main_redirect,submit,send_message,style,login,steamlogin,steam_redirector])
                .mount("/graphics", StaticFiles::from("Mods/SevenWebser/graphics"))
                .mount("/scripts", StaticFiles::from("Mods/SevenWebser/scripts"))
                .mount("/fonts", StaticFiles::from("Mods/SevenWebser/fonts"))
                .register(catchers![not_found])
                .launch();
        });
    }

    //tutaj testy:
    /*let names = dbus_send_chat();
    match names {
        Ok(list) => {
            log::info!("DBUS: {}", "Chat sending successfully");
        }
        Err(_) => {}
    }*/
}
// You can pass the handler as a function or a closure. In this
// case, we've chosen a function for clarity.
// Since we don't care about the request, we bind it to _.
/*fn handler(_: &mut Request) -> IronResult<Response> {
	Ok(Response::with((status::Ok, "Serwer 7dtd testowy:")))
}

pub fn start_main() {
	thread::spawn(|| {
		Iron::new(handler).http("0.0.0.0:8080").unwrap();
	});
}*/
