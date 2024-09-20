use regex::Regex;
use std::{collections::HashMap, env::args, fs::read_to_string};

fn main() {
    let args = args().collect::<Vec<String>>();
    if let Some(path) = args.get(1) {
        if let Ok(code) = read_to_string(path) {
            let mut openmind = Core {
                stack: vec![],
                memory: HashMap::new(),
            };
            openmind.eval(code);
        } else {
            eprintln!("エラー！ファイルが開けませんでした");
        };
    } else {
        println!("日本語プログラミング言語OpenMind");
    }
}

#[derive(Clone, Debug)]
enum Type {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Type {
    fn get_number(&self) -> f64 {
        match self {
            Type::Number(i) => i.to_owned(),
            Type::String(s) => s.trim().parse().unwrap_or_default(),
            Type::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }

    fn get_string(&self) -> String {
        match self {
            Type::Number(i) => i.to_string(),
            Type::String(s) => s.to_owned(),
            Type::Bool(b) => if *b { "真" } else { "偽" }.to_string(),
        }
    }

    fn get_bool(&self) -> bool {
        match self {
            Type::Number(i) => *i != 0.0,
            Type::String(s) => !s.is_empty(),
            Type::Bool(b) => *b,
        }
    }
}

#[derive(Clone, Debug)]
struct Core {
    stack: Vec<Type>,
    memory: HashMap<String, Type>,
}

impl Core {
    fn tokenize(soruce: String) -> Option<Vec<String>> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_parentheses: usize = 0;

        for c in soruce.chars() {
            match c {
                '「' => {
                    in_parentheses += 1;
                    current_token.push(c);
                }
                '」' => {
                    if in_parentheses != 0 {
                        current_token.push(c);
                        in_parentheses -= 1;
                        if in_parentheses == 0 {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                    }
                }
                other => {
                    if if let Ok(i) = Regex::new(
                        r"[あ-ん]|[ア-ン]|[a-z]|[A-Z]| |\n|\t|\r|　|,|、|。|\.|ー|\-|\~|〜|!|！|＾|\^|\?|？",
                    ) {
                        i
                    } else {
                        return None;
                    }
                    .is_match(&other.to_string())
                    {
                        if in_parentheses != 0 {
                            current_token.push(c);
                        } else if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                    } else {
                        current_token.push(c);
                    }
                }
            }
        }

        if !(in_parentheses != 0 || current_token.is_empty()) {
            tokens.push(current_token);
        }
        Some(tokens)
    }

    fn eval(&mut self, soruce: String) -> Option<()> {
        let tokens = Core::tokenize(soruce)?;
        for token in tokens.iter() {
            let token = token.trim().to_string();
            if token.is_empty() {
                continue;
            }

            if let Some(value) = self.memory.get(&token) {
                self.stack.push(value.to_owned());
            } else if let Ok(i) = token.parse::<f64>() {
                self.stack.push(Type::Number(i))
            } else if token == "真" {
                self.stack.push(Type::Bool(true));
            } else if token == "偽" {
                self.stack.push(Type::Bool(false));
            } else if token.starts_with("「") && token.ends_with("」") {
                let mut token = token.clone();
                token.remove(token.find("「")?);
                token.remove(token.rfind("」")?);
                self.stack.push(Type::String(token))
            } else {
                match token.as_str() {
                    "表示" => {
                        println!("{}", self.stack.pop()?.get_string());
                    }
                    "結合" => {
                        let str2 = self.stack.pop()?.get_string();
                        let str1 = self.stack.pop()?.get_string();
                        self.stack.push(Type::String(str1 + &str2));
                    }
                    "足" => {
                        let num2 = self.stack.pop()?.get_number();
                        let num1 = self.stack.pop()?.get_number();
                        self.stack.push(Type::Number(num1 + num2));
                    }
                    "引" => {
                        let num2 = self.stack.pop()?.get_number();
                        let num1 = self.stack.pop()?.get_number();
                        self.stack.push(Type::Number(num1 - num2));
                    }
                    "掛" => {
                        let num2 = self.stack.pop()?.get_number();
                        let num1 = self.stack.pop()?.get_number();
                        self.stack.push(Type::Number(num1 * num2));
                    }
                    "割" => {
                        let num2 = self.stack.pop()?.get_number();
                        let num1 = self.stack.pop()?.get_number();
                        self.stack.push(Type::Number(num1 / num2));
                    }
                    "余" => {
                        let num2 = self.stack.pop()?.get_number();
                        let num1 = self.stack.pop()?.get_number();
                        self.stack.push(Type::Number(num1 % num2));
                    }
                    "等" => {
                        let str1 = self.stack.pop()?.get_string();
                        let str2 = self.stack.pop()?.get_string();
                        self.stack.push(Type::Bool(str1 == str2));
                    }
                    "大" => {
                        let num2 = self.stack.pop()?.get_number();
                        let num1 = self.stack.pop()?.get_number();
                        self.stack.push(Type::Bool(num1 > num2));
                    }
                    "小" => {
                        let num2 = self.stack.pop()?.get_number();
                        let num1 = self.stack.pop()?.get_number();
                        self.stack.push(Type::Bool(num1 < num2));
                    }
                    "和" => {
                        let bool2 = self.stack.pop()?.get_bool();
                        let bool1 = self.stack.pop()?.get_bool();
                        self.stack.push(Type::Bool(bool1 || bool2));
                    }
                    "積" => {
                        let bool2 = self.stack.pop()?.get_bool();
                        let bool1 = self.stack.pop()?.get_bool();
                        self.stack.push(Type::Bool(bool1 && bool2));
                    }
                    "否" => {
                        let bool1 = self.stack.pop()?.get_bool();
                        self.stack.push(Type::Bool(!bool1));
                    }
                    "代入" => {
                        let name = self.stack.pop()?.get_string();
                        let value = self.stack.pop()?;
                        self.memory.insert(name, value);
                    }
                    "評価" => {
                        let code = self.stack.pop()?.get_string();
                        self.eval(code)?;
                    }
                    "条件分岐" => {
                        let code_false = self.stack.pop()?.get_string();
                        let code_true = self.stack.pop()?.get_string();
                        let condition = self.stack.pop()?.get_bool();
                        if condition {
                            self.eval(code_true)?;
                        } else {
                            self.eval(code_false)?;
                        }
                    }
                    "反復" => {
                        let code = self.stack.pop()?.get_string();
                        let condition = self.stack.pop()?.get_string();
                        while {
                            self.eval(condition.clone());
                            self.stack.pop()?.get_bool()
                        } {
                            self.eval(code.clone());
                        }
                    }
                    _ => return None,
                }
            }
        }
        Some(())
    }
}
