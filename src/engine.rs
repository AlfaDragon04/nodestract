#![allow(dead_code, unused_imports)]
#[path = "lexer/lexer.rs"]
pub mod lexer;
#[path = "ast.rs"]
pub mod ast;
#[path = "parser/parser.rs"]
pub mod parser;
#[path = "interpreter.rs"]
pub mod interpreter;
#[path = "value.rs"]
pub mod value;
#[path = "translate/translate.rs"]
pub mod translate;
#[path = "import/import.rs"]
pub mod import;

use self::lexer::{Lexer, Token};
use self::parser::Parser;
use self::interpreter::Interpreter;
use self::translate::TranslationEngine;
use self::import::ImportManager;

pub struct Engine {
    pub translation_engine: TranslationEngine,
    pub import_manager: ImportManager,
    pub interpreter: Interpreter,
}

impl Engine {
    /// Create a new Engine instance.
    pub fn new() -> Self {
        println!("[Engine] Initializing Translation Engine (supporting all active languages)...");
        let translation_engine = TranslationEngine::new();
        
        println!("[Engine] Initializing Import Manager...");
        let import_manager = ImportManager::new();

        println!("[Engine] Initializing Runtime Interpreter...");
        let interpreter = Interpreter::new();

        Self {
            translation_engine,
            import_manager,
            interpreter,
        }
    }

    /// Runs the complete NodeStract pipeline for a given source code.
    pub fn run(&mut self, source: &str) {
        println!("[Engine] [1/3] Extracting and validating imports (First Pass)...");
        
        // Reset active imports for this compilation run
        self.import_manager = ImportManager::new();

        // Tokenize once to extract import statements (keywords like `import` and `from` are always active)
        let mut lexer1 = Lexer::new(source);
        let first_pass_tokens = lexer1.tokenize(&self.translation_engine, &self.import_manager);

        // Find all imports in the token stream
        let mut imports = Vec::new();
        let mut i = 0;
        while i < first_pass_tokens.len() {
            if let Token::Keyword(ref kw) = first_pass_tokens[i] {
                if kw == "import" {
                    if i + 1 < first_pass_tokens.len() {
                        let member = match &first_pass_tokens[i + 1] {
                            Token::Operator(op) if op == "*" => Some("*".to_string()),
                            Token::Identifier(id) => Some(id.clone()),
                            Token::Keyword(k) => Some(k.clone()),
                            _ => None,
                        };
                        if let Some(m) = member {
                            if i + 2 < first_pass_tokens.len() {
                                if let Token::Keyword(ref from_kw) = first_pass_tokens[i + 2] {
                                    if from_kw == "from" {
                                        if i + 3 < first_pass_tokens.len() {
                                            if let Token::Identifier(ref parent) = first_pass_tokens[i + 3] {
                                                imports.push((m, parent.clone()));
                                                i += 4;
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }

        // Register and validate all found imports
        for (member, parent) in &imports {
            if !self.import_manager.import_member(member, parent) {
                crate::welcome::show_error(&format!("Import Error: Cannot import '{}' from '{}'", member, parent));
                println!("[Engine] Execution aborted due to import validation errors.");
                return;
            }
        }

        println!("[Engine] [2/3] Performing second pass tokenization with active imports...");
        let mut lexer2 = Lexer::new(source);
        let final_tokens = lexer2.tokenize(&self.translation_engine, &self.import_manager);

        // Verify that any built-in functions referenced in the code are actually imported
        for token in &final_tokens {
            if let Token::Identifier(ref name) = token {
                if let Some((canonical, module)) = self.translation_engine.get_builtin_info(name) {
                    if !self.import_manager.is_member_active(canonical, module) {
                        crate::welcome::show_error(&format!(
                            "Import Error: Built-in function '{}' used but its library module '{}' was not imported",
                            name, module
                        ));
                        println!("[Engine] Execution aborted due to missing imports.");
                        return;
                    }
                }
            }
        }

        println!("[Engine] [3/3] Starting Parser (Syntax Validation)...");
        let mut parser = Parser::new(final_tokens.clone());
        match parser.parse() {
            Ok(_) => {
                println!("[Engine] Syntax and imports are flawless! Displaying Tokens:");
                let token_reprs: Vec<String> = final_tokens
                    .iter()
                    .map(|t| t.to_string_repr())
                    .collect();
                
                println!("--------------------------------------------------");
                println!("[ {} ]", token_reprs.join(" | "));
                println!("--------------------------------------------------");
            }
            Err(err_msg) => {
                crate::welcome::show_error(&format!("Syntax Error: {}", err_msg));
                println!("[Engine] Execution aborted due to syntax errors.");
            }
        }
    }

    /// Reads a file from disk and runs the compiler pipeline.
    pub fn run_file(&mut self, filename: &str) {
        println!("[Engine] Loading file: {}", filename);
        match std::fs::read_to_string(filename) {
            Ok(content) => {
                self.run(&content);
                crate::welcome::show_success("Execution finished successfully.");
            }
            Err(_) => {
                crate::welcome::show_error(&format!("Could not read file '{}'. Check the path.", filename));
            }
        }
    }
}
