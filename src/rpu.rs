use crate::prelude::*;

pub struct RPU {
    pub world               : World,
    pub preview             : World,

    pub context             : Context,
    pub stack               : Vec<Value>,

    pub dictionary          : FxHashMap<String, Vec<Value>>,

    pub bake                : Option<Bake>,
}

impl RPU {
    pub fn new() -> Self {

        let world = World::new();
        let preview = World::new();

        let context: Context = Context::new();

        let dictionary = FxHashMap::default();

        Self {
            world,
            preview,

            context,
            stack           : vec![],

            dictionary,

            bake            : None,
        }
    }

    /// Reload all data
    pub fn reload(&mut self) {
        self.load_dictionary();
    }

    /// Process the given string
    pub fn process(&mut self, input: String, buffer: &mut ColorBuffer) -> (bool, Vec<String>) {

        let mut output_image = false;
        let mut output_text = vec![];
        let mut error = false;

        let rc = self.valuefy(input);

        if rc.is_err() {
            return (false, vec![rc.err().unwrap()]);
        }

        let mut values = rc.ok().unwrap();

        // STACK Config
        if values.is_empty() == false && values[0] == Value::Config("STACK".to_string()) {

            if self.stack.is_empty() {
                return (false, vec!["Stack is empty.".to_string()]);
            }

            let mut stack_messages = vec![];
            for (index, v) in self.stack.iter().enumerate() {
                stack_messages.push(format!("{}. {}", index + 1, v.to_string()));
            }
            return (false, stack_messages);
        }

        // DICT Config
        if values.is_empty() == false && values[0] == Value::Config("DICT".to_string()) {

            if self.dictionary.is_empty() {
                return (false, vec!["Dictionary is empty.".to_string()]);
            }

            let mut dict_messages = vec![];
            for (word, values) in &self.dictionary {
                let mut val = String::new();
                for v in values {
                    val += v.to_string().as_str();
                    val += " ";
                }
                dict_messages.push(format!("{}: {}", word, val));
            }
            return (false, dict_messages);
        }

        // Word definition
        if values.is_empty() == false && values[0] == Value::WordDefinitionStart() {
            _ = values.remove(0);

            if values.len() > 2 {
                let word = values.remove(0);
                let word_end = values.remove(values.len()-1);

                if word_end == Value::WordDefinitionEnd() {
                    self.dictionary.insert(word.to_string(), values);
                    self.save_dictionary();
                    return (false, vec![format!("{} added to the dictionary.", word.to_string())]);
                } else {
                    return (false, vec![format!("Invalid word definition.")]);
                }
            } else {
                return (false, vec![format!("Invalid word definition.")]);
            }
        }

        let mut cloned = values.clone();
        self.stack.append(&mut cloned);
        values = self.stack.clone();

        loop {
            if let Some(value) = values.pop() {
                match value {
                    Value::Shape3D(mut sdf) => {
                        if let Some(compile) = &mut self.bake {

                            let rc = sdf.read_properties(&mut values);
                            if rc.is_err() {
                                return (false, vec![rc.err().unwrap()]);
                            }

                            compile.sdf = Some(sdf);
                        }
                    },
                    Value::Command(cmd) => {
                        match cmd.as_str() {
                            "BAKE" => {
                                self.bake = Some(Bake::new());
                            },
                            _ => {
                            }
                        }
                    }
                    _ => {
                        //return (false, format!("Unexpected Value: {}", )
                    }
                }
            } else {
                break;
            }
        }

        if let Some(bake) = &self.bake {
            output_image = true;
            self.preview.clear();
            self.preview.compile(bake, &mut self.context);

            for i in 0..10 {
                self.preview.render(buffer, &mut self.context, i);
            }

            self.stack = vec![];
        }

        self.bake = None;

        /*
        loop {
            let token = scanner.scan_token(false);

            let kind = token.kind;

            if kind == TokenType::Colon {
                println!("cc {}", token.lexeme);
            } else
            if kind == TokenType::Eof {
                break;
            } else
            if kind == TokenType::Dot {
                if let Some(v) = self.stack.pop() {
                    output_text.push(v.to_string());
                } else {
                    output_text.push(format!("Stack is empty.").to_string());
                    error = true;
                }
            } else
            if kind == TokenType::Number {
                if let Some(n) = token.lexeme.parse::<f32>().ok() {
                    self.stack.push(Value::Number(n));
                }
            } else
            if kind == TokenType::Identifier {
                if token.lexeme == "BOX" {
                    let mut sdf = SDF3D::new(SDF3DType::Box);
                    if let Some(err) = sdf.read_properties(&mut self.stack).err() {
                        output_text.push(err);
                        error = true;
                    } else {
                        output_image = true;
                        self.preview.clear();
                        self.preview.compile(sdf, Vec3f::new(0.0, 0.5, 0.0));

                        for i in 0..1 {
                            self.preview.render(buffer, &mut self.context, i);
                        }
                    }
                } else
                if token.lexeme == "SPHERE" {
                    let mut sdf = SDF3D::new(SDF3DType::Sphere);
                    if let Some(err) = sdf.read_properties(&mut self.stack).err() {
                        output_text.push(err);
                        error = true;
                    } else {
                        output_image = true;
                        self.preview.clear();
                        self.preview.compile(sdf, Vec3f::new(0.0, 0.5, 0.0));

                        for i in 0..1 {
                            self.preview.render(buffer, &mut self.context, i);
                        }
                    }
                }
            }

            //output_text.push(format!("{:?}", token).to_string());
        }*/

        (output_image, output_text)
    }

    /// Create values out of the token stream
    pub fn valuefy(&mut self, input: String) -> Result<Vec<Value>, String> {
        let mut scanner = Scanner::new(input.trim().into());
        let mut values : Vec<Value> = vec![];

        let mut first_value: bool = true;
        let mut word_definition: bool = false;

        loop {
            let token = scanner.scan_token(false);
            let kind = token.kind;

            if kind == TokenType::Colon {
                values.push(Value::WordDefinitionStart());
                word_definition = true;
            } else
            if kind == TokenType::Semicolon {
                values.push(Value::WordDefinitionEnd());
            } else
            if kind == TokenType::Eof {
                break;
            } else
            if kind == TokenType::Number {
                if let Some(n) = token.lexeme.parse::<f32>().ok() {
                    values.push(Value::Number(n));
                }
            } else
            if kind == TokenType::LeftBracket {

                // Array can contain numbers only so far
                let mut array : Vec<Value> = vec![];

                loop {
                    let token = scanner.scan_token(false);
                    let kind = token.kind;

                    if kind == TokenType::RightBracket {
                        break;
                    } else
                    if kind == TokenType::Number {
                        if let Some(n) = token.lexeme.parse::<f32>().ok() {
                            array.push(Value::Number(n));
                        }
                    } else
                    if kind == TokenType::Eof {
                        return Err("Missing ']' after Array.".to_string());
                    } else {
                        return Err(format!("Unknown token in Array: {}.", token.lexeme));
                    }
                }
                values.push(Value::Array(array));
            } else
            if kind == TokenType::Identifier {
                if token.lexeme == "BOX" {
                    let sdf = SDF3D::new(SDF3DType::Box);
                    values.push(Value::Shape3D(sdf));
                } else
                if token.lexeme == "SPHERE" {
                    let sdf = SDF3D::new(SDF3DType::Sphere);
                    values.push(Value::Shape3D(sdf));
                } else
                if token.lexeme == "BAKE" {
                    values.push(Value::Command(token.lexeme));
                } else

                // Configs
                if first_value && token.lexeme == "STACK" {
                    values.push(Value::Config(token.lexeme));
                } else
                if first_value && (token.lexeme == "DICT" || token.lexeme == "DICTIONARY") {
                    values.push(Value::Config("DICT".to_string()));
                } else

                // Word Definition ?
                if word_definition {
                    values.push(Value::Config(token.lexeme));
                } else

                // In Dictionary ?
                if let Some(dict) = self.dictionary.get(&token.lexeme) {
                    let mut cloned = dict.clone();
                    values.append(&mut cloned);
                }

                else {
                    return Err(format!("Unknown identifier: {}", token.lexeme))
                }
            }

            first_value = false;
        }

        Ok(values)
    }

    pub fn render_world(&mut self, buffer: &mut ColorBuffer) {

        self.world.render(buffer, &mut self.context, 0);
        return;

        // let width = buffer.width;
        // let height = buffer.height as f32;

        // let screen = vec2f(buffer.width as f32, buffer.height as f32);

        //let time = (iteration as f32 * 1000.0 / 60.0) / 1000.0;
        //let _start = self.get_time();
        /*
        buffer.pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {

                    #[inline(always)]
                    pub fn _mix_color(a: &[f32], b: &[f32], v: f32) -> [f32; 4] {
                        [   (1.0 - v) * a[0] + b[0] * v,
                            (1.0 - v) * a[1] + b[1] * v,
                            (1.0 - v) * a[2] + b[2] * v,
                            (1.0 - v) * a[3] + b[3] * v ]
                    }

                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = height - (i / width) as f32;

                    let uv = vec2f(x / width as f32, 1.0 - (y / height));

                    let color = [uv.x, uv.y, 0.0, 1.0];

                    pixel.copy_from_slice(&color);
                }
        });*/

    }

    pub fn render_palette(&mut self, buffer: &mut ColorBuffer) {

        let width = buffer.width;
        let height = buffer.height as f32;

        // let screen = vec2f(buffer.width as f32, buffer.height as f32);

        // let time = (iteration as f32 * 1000.0 / 60.0) / 1000.0;
        // let _start = self.get_time();

        buffer.pixels
            .par_rchunks_exact_mut(width * 4)
            .enumerate()
            .for_each(|(j, line)| {
                for (i, pixel) in line.chunks_exact_mut(4).enumerate() {

                    #[inline(always)]
                    pub fn _mix_color(a: &[f32], b: &[f32], v: f32) -> [f32; 4] {
                        [   (1.0 - v) * a[0] + b[0] * v,
                            (1.0 - v) * a[1] + b[1] * v,
                            (1.0 - v) * a[2] + b[2] * v,
                            (1.0 - v) * a[3] + b[3] * v ]
                    }

                    let i = j * width + i;

                    let x = (i % width) as f32;
                    let y = height - (i / width) as f32;

                    let uv = vec2f(x / width as f32, 1.0 - (y / height));

                    let color = [uv.x, uv.y, 0.0, 1.0];

                    pixel.copy_from_slice(&color);
                }
        });

    }

    /// Load the dictionary from disk
    fn load_dictionary(&mut self) {
        if let Some(data) = std::fs::read_to_string("dictionary.json").ok() {
            if let Some(dict) = serde_json::from_str::<FxHashMap<String, Vec<Value>>>(&data).ok() {
                self.dictionary = dict;
            }
        }
    }

    /// Save the dictionary to disk
    fn save_dictionary(&self) {
        if let Some(json) = serde_json::to_string_pretty(&self.dictionary).ok() {
            std::fs::write("dictionary.json", json).expect("Unable to write dictionary");
        }
    }
}