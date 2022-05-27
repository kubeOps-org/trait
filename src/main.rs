trait StudyRoom {
    fn new() -> Self;
    fn studying(&self) -> String;
}

struct Lab {
    cando: bool,
}

struct Lecture {
    cando: bool,
}

impl StudyRoom for Lab {
    fn new() -> Self {
        Lab { cando: true }
    }

    fn studying(&self) -> String {
        let mut result: String = "".to_string();
        if self.cando {
            result = "CAN DO EVERY THING".to_string();
        }
        return result;
    }
}

impl StudyRoom for Lecture {
    fn new() -> Self {
        Lecture { cando: false }
    }

    fn studying(&self) -> String {
        let mut result: String = "".to_string();
        if !self.cando {
            result = String::from("CAN SIT STUDY ONLY").to_string();
        }
        return result;
    }
}

fn main() {
    let lab = Lab::new();
    let lecture = Lecture::new();

    println!("LAB : {} ", lab.studying().to_string());
    println!("LECTURE : {} ", lecture.studying().to_string());
}
