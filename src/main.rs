use crossterm::event::{read, Event, KeyCode, KeyModifiers, KeyEvent};
fn type_of<T>(_:T) -> &'static str{
    std::any::type_name::<T>()
}


fn main() -> std::io::Result<()>{
    loop {
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => {

                //{ code: Char('j'), modifiers: KeyModifiers(0x0), kind: Press, state: KeyEventState(0x0) }
                let KeyEvent {code, modifiers, ..} = event;
                
                match (code, modifiers) {
                    (KeyCode::Char(s), KeyModifiers::CONTROL) => {
                        println!("Modifier: {:?}, Key:s", modifiers);
                    },

                    (KeyCode::Up, _) => println!("Up"),
                    (KeyCode::Down, _) => println!("Down"),
                    (KeyCode::Left, _) => println!("Left"),
                    (KeyCode::Right, _) => println!("Right"),
                    (_, _) => {}

                }
                
            },
            Event::Mouse(event) => println!("{:?}", event),
            Event::Paste(data) => println!("{:?}", data),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
}
