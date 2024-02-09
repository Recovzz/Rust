// Message enum :
// Une énumération Message pour représenter les différents types de messages que notre application peut recevoir.
// Chaque variante de Message correspond à un type de message spécifique avec ses données associées. Les variantes sont les suivantes :
// ChangeColor(u8, u8, u8): Pour changer la couleur, trois valeurs RGB sont passées en tant que données associées.
// Echo(String): Pour l'écho, une chaîne de caractères est passée en tant que données associées.
// Move(Point): Pour le mouvement, une structure Point est passée en tant que données associées.
// Quit: Pour quitter l'application.

// State struct :
// Une structure State pour représenter l'état global de notre application.
// Cette structure contient des champs tels que color, position, quit, et message, qui représentent respectivement la couleur actuelle, la position, le statut d'arrêt et le message actuel de l'application.

// Nous définissons une énumération `Message` pour représenter les différents types de messages
enum Message {
    // Chaque variante correspond à un type de message spécifique avec ses données associées
    ChangeColor(u8, u8, u8), // Pour changer la couleur, trois valeurs RGB
    Echo(String),            // Pour l'écho, une chaîne de caractères
    Move(Point),             // Pour le mouvement, une structure `Point`
    Quit,                    
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    // Changement de la couleur
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    
    fn quit(&mut self) {
        self.quit = true;
    }

    
    fn echo(&mut self, s: String) {
        self.message = s;
    }

   
    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

 
    fn process(&mut self, message: Message) {
        // Expression `match` pour traiter les différentes variantes de message
        match message {
            Message::ChangeColor(color1, color2, color3) => self.change_color((color1, color2, color3)),
            Message::Echo(s) => self.echo(s),
            Message::Move(p) => self.move_position(p),
            Message::Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

      
        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}
