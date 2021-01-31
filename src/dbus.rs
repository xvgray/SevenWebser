use dbus::blocking::Connection;
use dbus;
use std::time::Duration;
use serde::export::Vec;

/*pub fn dbus_check_names() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // First open up a connection to the session bus.
    let conn = Connection::new_session()?;

    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    // Now make the method call. The ListNames method call takes zero input parameters and
    // one output parameter which is an array of strings.
    // Therefore the input is a zero tuple "()", and the output is a single tuple "(names,)".
    let (names, ): (Vec<String>, ) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    // Let's print all the names to stdout.
    //for name in names { println!("{}", name); }
    Ok(names)
}*/

pub fn dbus_send_chat(user: String, message: String) -> Result<i32, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    //let conn = Connection::new_session()?;
    let proxy = conn.with_proxy("com.webser.server", "/com/webser", Duration::from_millis(5000));
    let (names, ): (i32, ) = proxy.method_call("com.webser.commands", "ChatMessage", (user.as_str(), message.as_str()))?;

    Ok(names)
}

//----------------------------
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)] //potrzebne do sortowania struktury
pub struct Gracz {
    pub nazwa : String,
    pub player_id : String,
    pub is_admin : bool
}
pub fn dbus_get_online_players() -> Result<Vec<Gracz>, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let mut wyjscie: Vec<Gracz> = Vec::new();
    let proxy = conn.with_proxy("com.webser.server", "/com/webser", Duration::from_millis(5000));
    let (users, ): (Vec<String>, ) = proxy.method_call("com.webser.commands", "OnlinePlayers",())?;
    //tutaj trzeba zdekodowac zwrocony ciąg nazw, oddzielonych znakiem RETURN na wektor stringów
    for pozycja in users {
        //rozbijamy na 3 wektory
        let mut _users: Vec<String> = pozycja.split("\n").map(String::from).collect();
        let mut _gracz = Gracz {
            nazwa : _users[0].to_string(),
            player_id : _users[1].to_string(),
            is_admin : false
        };
        if _users[2] == "Admin" { _gracz.is_admin = true; }
        wyjscie.push(_gracz);
    }


    Ok(wyjscie)
}

/*pub fn dbus_get_online_players() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let proxy = conn.with_proxy("com.webser.server", "/com/webser", Duration::from_millis(5000));
    let (users, ): (String, ) = proxy.method_call("com.webser.commands", "OnlinePlayers",())?;
    //tutaj trzeba zdekodowac zwrocony ciąg nazw, oddzielonych znakiem RETURN na wektor stringów
    let mut _users : Vec<String> = users.split('\n').map(String::from).collect();
    Ok(_users)
}*/