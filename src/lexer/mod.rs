enum Section {
    Data,
    Code,
    None,
}

pub(crate) struct Lexer {
    pub(crate) data: Vec<String>,
    pub(crate) code: Vec<String>,
    current_section: Section,
}

impl Lexer {
    pub(crate) fn new() -> Lexer {
        Lexer {
            data: Vec::new(),
            code: Vec::new(),
            current_section: Section::None,
        }
    }

    pub(crate) fn translate_to_asm(&mut self, input: String) {
        match input {
            _ if input == "CODE" => self.current_section = Section::Code,
            _ if input == "DATA" => self.current_section = Section::Data,
            _ if input == "" => return,
            _ => match self.current_section {
                Section::Code => self.code.push(input),
                Section::Data => self.data.push(input),
                Section::None => panic!("Section not set for line: {}", input),
            },
        }
    }
}
