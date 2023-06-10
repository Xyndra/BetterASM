enum Section {
    Data,
    Code,
    None,
}

pub(crate) struct Lexer {
    pub(crate) data: Vec<String>,
    pub(crate) code: Vec<String>,
    pub(crate) comments_above: Vec<String>,
    current_section: Section,
}

impl Lexer {
    pub(crate) fn new() -> Lexer {
        Lexer {
            data: Vec::new(),
            code: Vec::new(),
            comments_above: Vec::new(),
            current_section: Section::None,
        }
    }

    pub(crate) fn translate_to_asm(&mut self, input: Vec<String>) {
        let instruction = input[0].clone();
        let comment = input[1].clone();
        if instruction == "" {
            match self.current_section {
                Section::Code => self.code.push(comment.clone()),
                Section::Data => self.data.push(comment.clone()),
                Section::None => self.comments_above.push(comment.clone()),
            }
        } else if comment == "" {
            match instruction {
                _ if instruction == "CODE" => self.current_section = Section::Code,
                _ if instruction == "DATA" => self.current_section = Section::Data,
                _ => match self.current_section {
                    Section::Code => self.code.push(instruction),
                    Section::Data => self.data.push(instruction),
                    Section::None => panic!("Section not set for line: {}", instruction),
                },
            }
        } else {
            match instruction {
                _ if instruction == "CODE" => self.current_section = Section::Code,
                _ if instruction == "DATA" => self.current_section = Section::Data,
                _ => match self.current_section {
                    Section::Code => self.code.push(instruction + " " + &comment),
                    Section::Data => self.data.push(instruction + " " + &comment),
                    Section::None => panic!("Section not set for line: {}", instruction + " " + &comment),
                },
            }
        }
    }
}
