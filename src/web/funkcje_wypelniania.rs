use crate::read_prefs::pobierz_ustawienia;
//use rocket::http::ext::IntoCollection;
//use std::borrow::Borrow;
use crate::web::struktura_chat::MessageChat;
//use std::sync::Mutex;
use crate::dbus::{Gracz, dbus_get_online_players};


pub fn _wypelnij_game_params_icons() -> String {
    let mut wyjscie = String::new();
    let mut wartosc;// = String::new();
    wyjscie.push_str(format!(r#"<div id="CANVAS_1"><table id="_GAME_PARAMS_ICONS">"#).as_str());
    //generujemy zawartosc tabeli
    let settingsy = pobierz_ustawienia().property;
    for p in settingsy {
        match p.name.to_ascii_lowercase().as_str() {
            "servervisibility" => {
                wartosc = String::from(r#"<img  src="/graphics/eye_hidden.svg" title="Hidden or Private Server" height="25">"#);
                if p.value.as_str() == "2" { wartosc = String::from(r#"<img  src="/graphics/eye_look.svg" title="Public Server" height="25">"#); }
                wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1">{}</td></tr>"#, "Visibility:", wartosc).as_str())
            }
            "servermaxplayercount" => { wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1">{}</td></tr>"#, "Max Players:", p.value).as_str()) }
            "gameworld" => { wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1"><div title="{}">{}</div></td></tr>"#, "World Type:", p.value, p.value).as_str()) }
            "dropondeath" => {
                wartosc = String::from(r#"<img  src="/graphics/nope.svg" title="Nothing" height="25">"#);
                if p.value.as_str() == "1" { wartosc = String::from(r#"<img  src="/graphics/backpack_toolbelt.svg" title="Everything" height="25">"#); } else if p.value.as_str() == "2" { wartosc = String::from(r#"<img src="/graphics/toolbelt.svg" title="Toolbelt" height="25">"#); } else if p.value.as_str() == "3" { wartosc = String::from(r#"<img src="/graphics/backpack.svg" title="Backpack" height="25">"#); } else if p.value.as_str() == "4" { wartosc = String::from(r#"<img src="/graphics/trash.svg" title="Delete all" height="25">"#); }
                wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1">{}</td></tr>"#, "Drop on death:", wartosc).as_str())
            }
            "enemydifficulty" => {
                wartosc = String::from(r#"Normal"#);
                if p.value.as_str() == "1" { wartosc = String::from(r#"Feral"#); }
                wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1">{}</td></tr>"#, "Enemy Level:", wartosc).as_str())
            }
            //"dropondeath" => { wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1">{}</td></tr>"#,"Drop on Death:",p.value).as_str()) }
            //"bedrollexpirytime" => { wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1">{}</td></tr>"#,"Bedroll Expiry:",p.value).as_str()) }
            "gamedifficulty" => {
                wartosc = String::from(r#"<img  src="/graphics/difficulty_0.svg" title="Scavenger" height="20">"#);
                if p.value.as_str() == "1" { wartosc = String::from(r#"<img  src="/graphics/difficulty_1.svg" title="Adventurer" height="20">"#); } else if p.value.as_str() == "2" { wartosc = String::from(r#"<img  src="/graphics/difficulty_2.svg" title="Nomad" height="20">"#); } else if p.value.as_str() == "3" { wartosc = String::from(r#"<img  src="/graphics/difficulty_3.svg" title="Warrior" height="20">"#); } else if p.value.as_str() == "4" { wartosc = String::from(r#"<img  src="/graphics/difficulty_4.svg" title="Survivalist" height="20">"#); } else if p.value.as_str() == "5" { wartosc = String::from(r#"<img  src="/graphics/difficulty_5.svg" title="Insane!" height="20">"#); }
                wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1">{}</td></tr>"#, "Difficulty:", wartosc).as_str())
            }
            "playerkillingmode" => {
                wartosc = String::from(r#"<img  src="/graphics/peace.svg" title="No Killing (PvE)" height="20">"#);
                if p.value.as_str() == "1" { wartosc = String::from(r#"Kill Allies Only"#); } else if p.value.as_str() == "2" { wartosc = String::from(r#"Kill Strangers Only"#); } else if p.value.as_str() == "3" { wartosc = String::from(r#"Kill Everyone!"#); }
                wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1">{}</td></tr>"#, "Player versus:", wartosc).as_str())
            }
            "landclaimexpirytime" => { wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1">{}</td></tr>"#, "LCB Time:", p.value).as_str()) }
            "airdropfrequency" => { wyjscie.push_str(format!(r#"<tr><td>{}</td><td class="_td1">{}</td></tr>"#, "Airdrops:", p.value).as_str()) }
            _ => {}
        }
    }
    wyjscie.push_str(format!(r#"</table></div>"#).as_str());
    wyjscie
}

//wypelniamy chat tablicą graczy online
pub fn _wypelnij_chat_users(mut users_vector: Vec<Gracz>) -> String {
    let mut wyjscie = String::new();
    #[allow(unused)]
    let mut user_type = String::from("chat_user_player");
    wyjscie.push_str(format!(r#"<div id="CANVAS_CHAT_USERS"><div id="CANVAS_CHAT_USERS_REF">"#).as_str());
    users_vector.sort(); //TODO sprawdzic czy sortowanie dziala
    for user in users_vector {
        if user.is_admin == true { user_type = "chat_user_admin".to_string(); }
        else {user_type = "chat_user_player".to_string();}
        let title = format!("({})[{}]",&user.player_id, &user.nazwa);
        wyjscie.push_str(format!(r#"<span title="{}" class="{}">{}</span><br>"#, title, user_type, &user.nazwa).as_str());
    }

    wyjscie.push_str(format!(r#"</div></div>"#).as_str());
    wyjscie
}

//wypelniamy chat wiadomościami
pub fn _wypelnij_chat(chat_vector: &Vec<MessageChat>) -> String {
    let mut wyjscie = String::new();
    let userzy = dbus_get_online_players().unwrap_or_default();
    wyjscie.push_str(format!(r#"<div id="CANVAS_CHAT"><div id="CANVAS_CHAT_REF">"#).as_str());

    /*  tutaj wywolujemy funkcję, ktora zwraca tablicę 1000 ostatnich wiadomości
        tablicę formatujemy do html  */
    let mut chat_user = String::new();
    //TODO rozwiazac kolorowanie skladni jesli pisze admin z gry
    for wartosc in chat_vector {
        if !String::is_empty(&wartosc.user) {
            match wartosc.user.as_str() {
                "Server" => {chat_user = String::from("chat_user_server");} //say command
                "Admin" => {chat_user = String::from("chat_user_admin");} //from web
                &_ => {
                    for gracz in &userzy { //jesli nazwa usera pasuje do nazwy gracza z listy i ten jest adminem
                        if gracz.nazwa == wartosc.user.as_str() && gracz.is_admin {
                            chat_user = String::from("chat_user_admin");
                        }
                        else if gracz.nazwa == wartosc.user.as_str() && !gracz.is_admin {
                            chat_user = String::from("chat_user_player");
                        }
                        /*if (gracz.is_admin) {chat_user = String::from("chat_user_admin");}
                        else {chat_user = String::from("chat_user_player");}*/
                    }

                } //others - split to admins and players
            }
            wyjscie.push_str(format!(r#"
                <span class="{}">{}</span>
                <span class="chat_type">({})</span>
                <span class="chat_date">({})</span><br>
                <span class="chat_msg">{}</span><br>"#, chat_user, &wartosc.user, &wartosc.etype, &wartosc.date, &wartosc.message).as_str());
        }
    }

    wyjscie.push_str(format!(r#"</div></div>"#).as_str());
    wyjscie
}