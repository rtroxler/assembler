use std::collections::HashMap;

pub struct CInstructionTranslator {
    pub dest_map: HashMap<&'static str, &'static str>,
    pub jump_map: HashMap<&'static str, &'static str>,
    pub comp_map: HashMap<&'static str, &'static str>,
}

impl CInstructionTranslator {
    pub fn new() -> CInstructionTranslator {
        let mut dest_map = HashMap::new();
        dest_map.insert("M", "001");
        dest_map.insert("D", "010");
        dest_map.insert("A", "100");
        dest_map.insert("MD", "011");
        dest_map.insert("AM", "101");
        dest_map.insert("AD", "110");
        dest_map.insert("AMD", "111");

        let mut jump_map = HashMap::new();
        jump_map.insert("JGT", "001");
        jump_map.insert("JEQ", "010");
        jump_map.insert("JGE", "100");
        jump_map.insert("JLT", "011");
        jump_map.insert("JNE", "101");
        jump_map.insert("JLE", "110");
        jump_map.insert("JMP", "111");

        let mut comp_map = HashMap::new();
        comp_map.insert("0", "0101010");
        comp_map.insert("1", "0111111");
        comp_map.insert("-1", "0111010");
        comp_map.insert("D", "0001100");
        comp_map.insert("A", "0110000");
        comp_map.insert("!D", "0001101");
        comp_map.insert("!A", "0110001");
        comp_map.insert("-D", "0001111");
        comp_map.insert("-A", "0110011");
        comp_map.insert("D+1", "0011111");
        comp_map.insert("A+1", "0110111");
        comp_map.insert("A+1", "0110111");
        comp_map.insert("D-1", "0001110");
        comp_map.insert("A-1", "0110010");
        comp_map.insert("D+A", "0000010");
        comp_map.insert("D-A", "0010011");
        comp_map.insert("A-D", "0000111");
        comp_map.insert("D&A", "0000000");
        comp_map.insert("D|A", "0010101");
        comp_map.insert("M", "1110000");
        comp_map.insert("!M", "1110001");
        comp_map.insert("-M", "1110011");
        comp_map.insert("M+1", "1110111");
        comp_map.insert("M-1", "1110010");
        comp_map.insert("D+M", "1000010");
        comp_map.insert("D-M", "1010011");
        comp_map.insert("M-D", "1000111");
        comp_map.insert("D&M", "1000000");
        comp_map.insert("D|M", "1010101");

        CInstructionTranslator {
            dest_map: dest_map,
            jump_map: jump_map,
            comp_map: comp_map,
        }
    }
}
