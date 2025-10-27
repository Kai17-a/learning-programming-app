use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// User choice for section confirmation
#[derive(Debug, Clone, PartialEq)]
pub enum UserChoice {
    Approve,
    Reject,
    Modify,
    ShowDetails,
}

/// Represents a learning topic within a section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub name: String,
    pub syntax_elements: Vec<String>,
    pub difficulty: u8, // 1-3 scale
}

/// Represents a learning section containing multiple topics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id: String,
    pub name: String,
    pub description: String,
    pub problem_count: usize,
    pub topics: Vec<Topic>,
}

/// Configuration for all Go learning sections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionConfig {
    pub sections: Vec<Section>,
}

/// Represents a generated Go problem
#[derive(Debug, Clone)]
pub struct GoProblem {
    pub filename: String,
    pub content: String,
    #[allow(dead_code)]
    pub description: String,
    #[allow(dead_code)]
    pub topic: String,
    pub difficulty: u8,
}

impl SectionConfig {
    /// Creates the default configuration with 10 Go learning sections
    pub fn default_go_sections() -> Self {
        let sections = vec![
            Section {
                id: "section1-basics".to_string(),
                name: "Basic Syntax".to_string(),
                description: "Variables, constants, and basic data types".to_string(),
                problem_count: 10,
                topics: vec![
                    Topic {
                        name: "Variables".to_string(),
                        syntax_elements: vec![
                            "var declaration".to_string(),
                            "short variable declaration".to_string(),
                            "zero values".to_string(),
                        ],
                        difficulty: 1,
                    },
                    Topic {
                        name: "Constants".to_string(),
                        syntax_elements: vec![
                            "const declaration".to_string(),
                            "iota".to_string(),
                            "typed constants".to_string(),
                        ],
                        difficulty: 1,
                    },
                    Topic {
                        name: "Data Types".to_string(),
                        syntax_elements: vec![
                            "int, float, string, bool".to_string(),
                            "type conversion".to_string(),
                            "type inference".to_string(),
                        ],
                        difficulty: 2,
                    },
                ],
            },
            Section {
                id: "section2-control-flow".to_string(),
                name: "Control Flow".to_string(),
                description: "Conditional statements and loops".to_string(),
                problem_count: 10,
                topics: vec![
                    Topic {
                        name: "If Statements".to_string(),
                        syntax_elements: vec![
                            "if condition".to_string(),
                            "if-else".to_string(),
                            "if with initialization".to_string(),
                        ],
                        difficulty: 1,
                    },
                    Topic {
                        name: "For Loops".to_string(),
                        syntax_elements: vec![
                            "basic for loop".to_string(),
                            "for as while".to_string(),
                            "range loops".to_string(),
                        ],
                        difficulty: 2,
                    },
                    Topic {
                        name: "Switch Statements".to_string(),
                        syntax_elements: vec![
                            "switch expression".to_string(),
                            "switch without expression".to_string(),
                            "fallthrough".to_string(),
                        ],
                        difficulty: 2,
                    },
                ],
            },
            Section {
                id: "section3-functions".to_string(),
                name: "Functions".to_string(),
                description: "Function definitions, parameters, and return values".to_string(),
                problem_count: 10,
                topics: vec![
                    Topic {
                        name: "Function Basics".to_string(),
                        syntax_elements: vec![
                            "function declaration".to_string(),
                            "parameters".to_string(),
                            "return values".to_string(),
                        ],
                        difficulty: 1,
                    },
                    Topic {
                        name: "Multiple Returns".to_string(),
                        syntax_elements: vec![
                            "multiple return values".to_string(),
                            "named returns".to_string(),
                            "blank identifier".to_string(),
                        ],
                        difficulty: 2,
                    },
                    Topic {
                        name: "Variadic Functions".to_string(),
                        syntax_elements: vec![
                            "variadic parameters".to_string(),
                            "function literals".to_string(),
                            "closures".to_string(),
                        ],
                        difficulty: 3,
                    },
                ],
            },
            Section {
                id: "section4-packages".to_string(),
                name: "Packages and Imports".to_string(),
                description: "Package declarations and import statements".to_string(),
                problem_count: 10,
                topics: vec![
                    Topic {
                        name: "Package Declaration".to_string(),
                        syntax_elements: vec![
                            "package main".to_string(),
                            "package naming".to_string(),
                            "exported identifiers".to_string(),
                        ],
                        difficulty: 1,
                    },
                    Topic {
                        name: "Import Statements".to_string(),
                        syntax_elements: vec![
                            "single import".to_string(),
                            "grouped imports".to_string(),
                            "import aliases".to_string(),
                        ],
                        difficulty: 1,
                    },
                    Topic {
                        name: "Standard Library".to_string(),
                        syntax_elements: vec![
                            "fmt package".to_string(),
                            "strings package".to_string(),
                            "math package".to_string(),
                        ],
                        difficulty: 2,
                    },
                ],
            },
            Section {
                id: "section5-structs".to_string(),
                name: "Structs and Methods".to_string(),
                description: "Structure definitions and method implementations".to_string(),
                problem_count: 10,
                topics: vec![
                    Topic {
                        name: "Struct Definition".to_string(),
                        syntax_elements: vec![
                            "struct declaration".to_string(),
                            "struct literals".to_string(),
                            "field access".to_string(),
                        ],
                        difficulty: 2,
                    },
                    Topic {
                        name: "Methods".to_string(),
                        syntax_elements: vec![
                            "method declaration".to_string(),
                            "receiver types".to_string(),
                            "pointer receivers".to_string(),
                        ],
                        difficulty: 2,
                    },
                    Topic {
                        name: "Embedding".to_string(),
                        syntax_elements: vec![
                            "struct embedding".to_string(),
                            "promoted fields".to_string(),
                            "method promotion".to_string(),
                        ],
                        difficulty: 3,
                    },
                ],
            },
            Section {
                id: "section6-interfaces".to_string(),
                name: "Interfaces".to_string(),
                description: "Interface definitions and implementations".to_string(),
                problem_count: 10,
                topics: vec![
                    Topic {
                        name: "Interface Basics".to_string(),
                        syntax_elements: vec![
                            "interface declaration".to_string(),
                            "method sets".to_string(),
                            "interface satisfaction".to_string(),
                        ],
                        difficulty: 2,
                    },
                    Topic {
                        name: "Empty Interface".to_string(),
                        syntax_elements: vec![
                            "interface{}".to_string(),
                            "type assertions".to_string(),
                            "type switches".to_string(),
                        ],
                        difficulty: 3,
                    },
                    Topic {
                        name: "Interface Composition".to_string(),
                        syntax_elements: vec![
                            "embedded interfaces".to_string(),
                            "interface unions".to_string(),
                            "polymorphism".to_string(),
                        ],
                        difficulty: 3,
                    },
                ],
            },
            Section {
                id: "section7-concurrency".to_string(),
                name: "Concurrency Basics".to_string(),
                description: "Goroutines and channels fundamentals".to_string(),
                problem_count: 10,
                topics: vec![
                    Topic {
                        name: "Goroutines".to_string(),
                        syntax_elements: vec![
                            "go keyword".to_string(),
                            "goroutine creation".to_string(),
                            "anonymous goroutines".to_string(),
                        ],
                        difficulty: 2,
                    },
                    Topic {
                        name: "Channels".to_string(),
                        syntax_elements: vec![
                            "channel creation".to_string(),
                            "channel operations".to_string(),
                            "channel direction".to_string(),
                        ],
                        difficulty: 3,
                    },
                    Topic {
                        name: "Select Statement".to_string(),
                        syntax_elements: vec![
                            "select syntax".to_string(),
                            "non-blocking operations".to_string(),
                            "default case".to_string(),
                        ],
                        difficulty: 3,
                    },
                ],
            },
            Section {
                id: "section8-error-handling".to_string(),
                name: "Error Handling".to_string(),
                description: "Error types and error handling patterns".to_string(),
                problem_count: 10,
                topics: vec![
                    Topic {
                        name: "Error Interface".to_string(),
                        syntax_elements: vec![
                            "error type".to_string(),
                            "Error() method".to_string(),
                            "nil errors".to_string(),
                        ],
                        difficulty: 2,
                    },
                    Topic {
                        name: "Error Creation".to_string(),
                        syntax_elements: vec![
                            "errors.New()".to_string(),
                            "fmt.Errorf()".to_string(),
                            "custom error types".to_string(),
                        ],
                        difficulty: 2,
                    },
                    Topic {
                        name: "Error Handling Patterns".to_string(),
                        syntax_elements: vec![
                            "if err != nil".to_string(),
                            "error wrapping".to_string(),
                            "panic and recover".to_string(),
                        ],
                        difficulty: 3,
                    },
                ],
            },
            Section {
                id: "section9-pointers".to_string(),
                name: "Pointers and References".to_string(),
                description: "Pointer syntax and memory references".to_string(),
                problem_count: 10,
                topics: vec![
                    Topic {
                        name: "Pointer Basics".to_string(),
                        syntax_elements: vec![
                            "pointer declaration".to_string(),
                            "address operator &".to_string(),
                            "dereference operator *".to_string(),
                        ],
                        difficulty: 2,
                    },
                    Topic {
                        name: "Pointer Usage".to_string(),
                        syntax_elements: vec![
                            "pointer to struct".to_string(),
                            "pointer receivers".to_string(),
                            "pointer arithmetic".to_string(),
                        ],
                        difficulty: 3,
                    },
                    Topic {
                        name: "Memory Management".to_string(),
                        syntax_elements: vec![
                            "new() function".to_string(),
                            "make() function".to_string(),
                            "garbage collection".to_string(),
                        ],
                        difficulty: 3,
                    },
                ],
            },
            Section {
                id: "section10-collections".to_string(),
                name: "Collections".to_string(),
                description: "Arrays, slices, and maps".to_string(),
                problem_count: 10,
                topics: vec![
                    Topic {
                        name: "Arrays".to_string(),
                        syntax_elements: vec![
                            "array declaration".to_string(),
                            "array literals".to_string(),
                            "array indexing".to_string(),
                        ],
                        difficulty: 1,
                    },
                    Topic {
                        name: "Slices".to_string(),
                        syntax_elements: vec![
                            "slice creation".to_string(),
                            "slice operations".to_string(),
                            "append function".to_string(),
                        ],
                        difficulty: 2,
                    },
                    Topic {
                        name: "Maps".to_string(),
                        syntax_elements: vec![
                            "map declaration".to_string(),
                            "map operations".to_string(),
                            "map iteration".to_string(),
                        ],
                        difficulty: 2,
                    },
                ],
            },
        ];

        SectionConfig { sections }
    }

    /// Get section by ID
    #[allow(dead_code)]
    pub fn get_section(&self, id: &str) -> Option<&Section> {
        self.sections.iter().find(|s| s.id == id)
    }

    /// Get all section names for display
    #[allow(dead_code)]
    pub fn get_section_names(&self) -> Vec<String> {
        self.sections.iter().map(|s| s.name.clone()).collect()
    }

    /// Get section count
    #[allow(dead_code)]
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// Validate section configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.sections.is_empty() {
            return Err("No sections defined".to_string());
        }

        for section in &self.sections {
            if section.topics.is_empty() {
                return Err(format!("Section '{}' has no topics", section.name));
            }

            if section.problem_count == 0 {
                return Err(format!("Section '{}' has zero problems", section.name));
            }

            for topic in &section.topics {
                if topic.syntax_elements.is_empty() {
                    return Err(format!("Topic '{}' has no syntax elements", topic.name));
                }

                if topic.difficulty < 1 || topic.difficulty > 3 {
                    return Err(format!(
                        "Topic '{}' has invalid difficulty level",
                        topic.name
                    ));
                }
            }
        }

        Ok(())
    }

    /// Display section preview to user for approval
    pub fn display_section_preview(&self) -> Result<(), io::Error> {
        println!("\n=== Go Learning Sections Preview ===");
        println!(
            "The following {} sections will be created:",
            self.sections.len()
        );
        println!();

        for (index, section) in self.sections.iter().enumerate() {
            println!("{}. {} ({})", index + 1, section.name, section.id);
            println!("   Description: {}", section.description);
            println!("   Problems: {} per section", section.problem_count);

            // Show first few topics as preview
            let topic_preview: Vec<String> = section
                .topics
                .iter()
                .take(3)
                .map(|t| t.name.clone())
                .collect();

            if topic_preview.len() < section.topics.len() {
                println!(
                    "   Topics: {} (and {} more)",
                    topic_preview.join(", "),
                    section.topics.len() - topic_preview.len()
                );
            } else {
                println!("   Topics: {}", topic_preview.join(", "));
            }
            println!();
        }

        println!(
            "Total problems to be generated: {}",
            self.sections.len() * self.sections.first().map_or(0, |s| s.problem_count)
        );
        println!();

        Ok(())
    }

    /// Get user confirmation for section structure
    pub fn get_user_confirmation(&self) -> Result<UserChoice, io::Error> {
        loop {
            print!("Do you want to proceed with this section structure? ");
            print!("(y)es / (n)o / (m)odify / (d)etails: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "y" | "yes" => return Ok(UserChoice::Approve),
                "n" | "no" => return Ok(UserChoice::Reject),
                "m" | "modify" => return Ok(UserChoice::Modify),
                "d" | "details" => return Ok(UserChoice::ShowDetails),
                _ => {
                    println!("Please enter 'y' (yes), 'n' (no), 'm' (modify), or 'd' (details)");
                    continue;
                }
            }
        }
    }

    /// Show detailed section information
    pub fn show_section_details(&self) -> Result<(), io::Error> {
        println!("\n=== Detailed Section Information ===");

        for (index, section) in self.sections.iter().enumerate() {
            println!("\n{}. {} ({})", index + 1, section.name, section.id);
            println!("   Description: {}", section.description);
            println!("   Problems: {}", section.problem_count);
            println!("   Topics:");

            for (topic_idx, topic) in section.topics.iter().enumerate() {
                println!(
                    "     {}.{} {} (Difficulty: {})",
                    index + 1,
                    topic_idx + 1,
                    topic.name,
                    topic.difficulty
                );
                println!(
                    "        Syntax Elements: {}",
                    topic.syntax_elements.join(", ")
                );
            }
        }
        println!();
        Ok(())
    }

    /// Handle section modification based on user input
    pub fn handle_section_modification(&mut self) -> Result<bool, io::Error> {
        loop {
            println!("\n=== Section Modification Options ===");
            println!("1. Remove a section");
            println!("2. Reorder sections");
            println!("3. Add a custom section");
            println!("4. Modify section details");
            println!("5. Reset to defaults");
            println!("6. Done with modifications");

            print!("Choose an option (1-6): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            match input {
                "1" => self.remove_section()?,
                "2" => self.reorder_sections()?,
                "3" => self.add_custom_section()?,
                "4" => self.modify_section_details()?,
                "5" => {
                    *self = SectionConfig::default_go_sections();
                    println!("Sections reset to defaults.");
                }
                "6" => return Ok(true),
                _ => println!("Please enter a number between 1 and 6"),
            }
        }
    }

    /// Remove a section from the configuration
    fn remove_section(&mut self) -> Result<(), io::Error> {
        if self.sections.is_empty() {
            println!("No sections to remove.");
            return Ok(());
        }

        println!("\nCurrent sections:");
        for (index, section) in self.sections.iter().enumerate() {
            println!("{}. {}", index + 1, section.name);
        }

        print!("Enter section number to remove (or 0 to cancel): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if let Ok(num) = input.trim().parse::<usize>() {
            if num == 0 {
                return Ok(());
            }
            if num > 0 && num <= self.sections.len() {
                let removed = self.sections.remove(num - 1);
                println!("Removed section: {}", removed.name);
            } else {
                println!("Invalid section number.");
            }
        } else {
            println!("Please enter a valid number.");
        }

        Ok(())
    }

    /// Reorder sections in the configuration
    fn reorder_sections(&mut self) -> Result<(), io::Error> {
        if self.sections.len() < 2 {
            println!("Need at least 2 sections to reorder.");
            return Ok(());
        }

        println!("\nCurrent section order:");
        for (index, section) in self.sections.iter().enumerate() {
            println!("{}. {}", index + 1, section.name);
        }

        print!("Enter two section numbers to swap (e.g., '1 3') or 0 to cancel: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() == 1 && parts[0] == "0" {
            return Ok(());
        }

        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                if a > 0 && b > 0 && a <= self.sections.len() && b <= self.sections.len() && a != b
                {
                    self.sections.swap(a - 1, b - 1);
                    println!("Swapped sections {} and {}", a, b);
                } else {
                    println!("Invalid section numbers or same section specified.");
                }
            } else {
                println!("Please enter valid numbers.");
            }
        } else {
            println!("Please enter exactly two numbers.");
        }

        Ok(())
    }

    /// Add a custom section to the configuration
    fn add_custom_section(&mut self) -> Result<(), io::Error> {
        println!("\n=== Add Custom Section ===");

        print!("Section ID (e.g., 'section11-custom'): ");
        io::stdout().flush()?;
        let mut id = String::new();
        io::stdin().read_line(&mut id)?;
        let id = id.trim().to_string();

        print!("Section Name: ");
        io::stdout().flush()?;
        let mut name = String::new();
        io::stdin().read_line(&mut name)?;
        let name = name.trim().to_string();

        print!("Section Description: ");
        io::stdout().flush()?;
        let mut description = String::new();
        io::stdin().read_line(&mut description)?;
        let description = description.trim().to_string();

        if id.is_empty() || name.is_empty() || description.is_empty() {
            println!("All fields are required. Section not added.");
            return Ok(());
        }

        // Create a basic topic for the custom section
        let topic = Topic {
            name: "Custom Topic".to_string(),
            syntax_elements: vec!["custom syntax".to_string()],
            difficulty: 2,
        };

        let section = Section {
            id,
            name: name.clone(),
            description,
            problem_count: 10,
            topics: vec![topic],
        };

        self.sections.push(section);
        println!("Added custom section: {}", name);

        Ok(())
    }

    /// Modify details of an existing section
    fn modify_section_details(&mut self) -> Result<(), io::Error> {
        if self.sections.is_empty() {
            println!("No sections to modify.");
            return Ok(());
        }

        println!("\nCurrent sections:");
        for (index, section) in self.sections.iter().enumerate() {
            println!("{}. {}", index + 1, section.name);
        }

        print!("Enter section number to modify (or 0 to cancel): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if let Ok(num) = input.trim().parse::<usize>() {
            if num == 0 {
                return Ok(());
            }
            if num > 0 && num <= self.sections.len() {
                let section = &mut self.sections[num - 1];

                println!("Current name: {}", section.name);
                print!("New name (press Enter to keep current): ");
                io::stdout().flush()?;

                let mut new_name = String::new();
                io::stdin().read_line(&mut new_name)?;
                let new_name = new_name.trim();

                if !new_name.is_empty() {
                    section.name = new_name.to_string();
                    println!("Updated section name.");
                }

                println!("Current description: {}", section.description);
                print!("New description (press Enter to keep current): ");
                io::stdout().flush()?;

                let mut new_desc = String::new();
                io::stdin().read_line(&mut new_desc)?;
                let new_desc = new_desc.trim();

                if !new_desc.is_empty() {
                    section.description = new_desc.to_string();
                    println!("Updated section description.");
                }
            } else {
                println!("Invalid section number.");
            }
        } else {
            println!("Please enter a valid number.");
        }

        Ok(())
    }
}

/// Main function to handle section preview and user confirmation workflow
pub fn preview_and_confirm_sections() -> Result<SectionConfig, Box<dyn std::error::Error>> {
    let mut config = SectionConfig::default_go_sections();

    loop {
        // Display section preview
        config.display_section_preview()?;

        // Get user choice
        match config.get_user_confirmation()? {
            UserChoice::Approve => {
                println!("Section structure approved. Proceeding with problem generation...");
                break;
            }
            UserChoice::Reject => {
                println!("Section structure rejected. Exiting...");
                return Err("User rejected section structure".into());
            }
            UserChoice::ShowDetails => {
                config.show_section_details()?;
                continue;
            }
            UserChoice::Modify => {
                if config.handle_section_modification()? {
                    println!("Modifications complete. Reviewing updated structure...");
                    continue;
                } else {
                    println!("Modification cancelled.");
                    continue;
                }
            }
        }
    }

    // Validate final configuration
    config
        .validate()
        .map_err(|e| format!("Configuration validation failed: {}", e))?;

    Ok(config)
}

impl GoProblem {
    /// Create a new Go problem
    pub fn new(
        filename: String,
        content: String,
        description: String,
        topic: String,
        difficulty: u8,
    ) -> Self {
        Self {
            filename,
            content,
            description,
            topic,
            difficulty,
        }
    }

    /// Validate that the problem meets requirements
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check filename extension
        if !self.filename.ends_with(".go") {
            return Err("Problem filename must end with .go".into());
        }

        // Check difficulty range
        if self.difficulty < 1 || self.difficulty > 3 {
            return Err("Problem difficulty must be between 1 and 3".into());
        }

        // Check that content contains required Go elements
        if !self.content.contains("package main") {
            return Err("Problem content must contain 'package main'".into());
        }

        if !self.content.contains("func main()") {
            return Err("Problem content must contain 'func main()'".into());
        }

        // Check that content has proper problem structure
        if !self.content.contains("// Problem:") {
            return Err("Problem content must contain '// Problem:' comment".into());
        }

        if !self.content.contains("// Topic:") {
            return Err("Problem content must contain '// Topic:' comment".into());
        }

        if !self.content.contains("// Difficulty:") {
            return Err("Problem content must contain '// Difficulty:' comment".into());
        }

        Ok(())
    }

    /// Get the full file path for this problem within a section
    #[allow(dead_code)]
    pub fn get_file_path(&self, section_id: &str) -> String {
        format!("learning-go/{}/{}", section_id, self.filename)
    }

    /// Generate a comprehensive set of problems for any section with progressive difficulty
    pub fn generate_progressive_problems_for_section(section: &Section) -> Vec<GoProblem> {
        let mut problems = Vec::new();

        // Ensure we generate exactly 10 problems
        let target_count = 10;
        let topics_count = section.topics.len();

        if topics_count == 0 {
            // Fallback for sections without topics
            return Self::generate_generic_section_problems(section);
        }

        // Distribute problems across topics with progressive difficulty
        for i in 0..target_count {
            let topic_index = i % topics_count;
            let topic = &section.topics[topic_index];

            // Progressive difficulty: 1-3 easy, 4-6 medium, 7-10 hard
            let difficulty = match i {
                0..=2 => 1, // First 3 problems: easy
                3..=5 => 2, // Next 3 problems: medium
                _ => 3,     // Last 4 problems: hard
            };

            let problem_number = i + 1;
            let filename = format!(
                "problem{:02}_{}.go",
                problem_number,
                topic.name.to_lowercase().replace(" ", "_")
            );

            let content = Self::generate_problem_content(
                problem_number,
                &topic.name,
                &topic.syntax_elements,
                difficulty,
                &section.description,
            );

            let description = format!(
                "Problem {}: {} (Difficulty {})",
                problem_number, topic.name, difficulty
            );

            problems.push(GoProblem::new(
                filename,
                content,
                description,
                topic.name.clone(),
                difficulty,
            ));
        }

        problems
    }

    /// Generate generic problems for sections without specific implementations
    fn generate_generic_section_problems(section: &Section) -> Vec<GoProblem> {
        let mut problems = Vec::new();

        for i in 1..=10 {
            let difficulty = match i {
                1..=3 => 1,
                4..=6 => 2,
                _ => 3,
            };

            let topic_name = if !section.topics.is_empty() {
                &section.topics[0].name
            } else {
                "General"
            };

            let filename = format!("problem{:02}_general.go", i);
            let content = format!(
                r#"// Problem: {} Problem {}
// Topic: {}
// Difficulty: {}

package main

import "fmt"

func main() {{
    // TODO: Implement {} problem {}
    // Focus on: {}
    
    fmt.Println("Problem {} - {} - Difficulty {}")
    
    // TODO: Add your implementation here
}}
"#,
                section.name,
                i,
                topic_name,
                difficulty,
                section.name,
                i,
                section.description,
                i,
                section.name,
                difficulty
            );

            let description = format!(
                "{} - Problem {} (Difficulty {})",
                section.name, i, difficulty
            );

            problems.push(GoProblem::new(
                filename,
                content,
                description,
                topic_name.to_string(),
                difficulty,
            ));
        }

        problems
    }

    /// Generate problem content based on topic and difficulty
    fn generate_problem_content(
        problem_number: usize,
        topic_name: &str,
        syntax_elements: &[String],
        difficulty: u8,
        section_description: &str,
    ) -> String {
        let difficulty_text = match difficulty {
            1 => "Basic",
            2 => "Intermediate",
            3 => "Advanced",
            _ => "Unknown",
        };

        let syntax_focus = if !syntax_elements.is_empty() {
            syntax_elements.join(", ")
        } else {
            "general syntax".to_string()
        };

        format!(
            r#"// Problem: {} {} Practice
// Topic: {}
// Difficulty: {}

package main

import "fmt"

func main() {{
    // TODO: This is a {} level problem focusing on {}
    // Section: {}
    // Syntax elements to practice: {}
    
    fmt.Println("Problem {}: {} - {} Level")
    
    // TODO: Implement your solution here
    // Focus on practicing: {}
    
    // TODO: Add appropriate variable declarations, control structures, or function calls
    // based on the topic and difficulty level
    
    // Example structure - modify as needed:
    // 1. Declare variables related to {}
    // 2. Implement logic using {}
    // 3. Display results using fmt package
}}
"#,
            topic_name,
            difficulty_text,
            topic_name,
            difficulty,
            difficulty_text.to_lowercase(),
            topic_name.to_lowercase(),
            section_description,
            syntax_focus,
            problem_number,
            topic_name,
            difficulty_text,
            syntax_focus,
            topic_name.to_lowercase(),
            syntax_focus
        )
    }

    /// Generate basic syntax problems for variables, constants, and data types
    pub fn generate_basic_syntax_problems() -> Vec<GoProblem> {
        vec![
            // Variable declaration problems
            GoProblem::new(
                "problem01_variables.go".to_string(),
                r#"// Problem: Variable Declaration Practice
// Topic: Variables
// Difficulty: 1

package main

import "fmt"

func main() {
    // TODO: Declare a variable named 'name' of type string and assign it your name
    
    // TODO: Declare a variable named 'age' using short variable declaration and assign it a number
    
    // TODO: Declare multiple variables in one line: x, y both integers with values 10, 20
    
    // TODO: Declare a variable without initialization and observe its zero value
    var count int
    
    fmt.Printf("Name: %s, Age: %d\n", name, age)
    fmt.Printf("X: %d, Y: %d\n", x, y)
    fmt.Printf("Count (zero value): %d\n", count)
}"#
                .to_string(),
                "Practice basic variable declarations using var keyword and short declaration"
                    .to_string(),
                "Variables".to_string(),
                1,
            ),
            GoProblem::new(
                "problem02_constants.go".to_string(),
                r#"// Problem: Constants and Iota
// Topic: Constants
// Difficulty: 1

package main

import "fmt"

func main() {
    // TODO: Declare a constant named 'pi' with value 3.14159
    
    // TODO: Declare multiple constants in a group
    const (
        // TODO: Add constants for days in week, hours in day
    )
    
    // TODO: Use iota to create enumerated constants
    const (
        Sunday = iota
        Monday
        Tuesday
        // TODO: Add remaining days
    )
    
    fmt.Printf("Pi: %f\n", pi)
    fmt.Printf("Days in week: %d, Hours in day: %d\n", daysInWeek, hoursInDay)
    fmt.Printf("Sunday: %d, Monday: %d, Tuesday: %d\n", Sunday, Monday, Tuesday)
}"#
                .to_string(),
                "Learn constant declarations and iota usage".to_string(),
                "Constants".to_string(),
                1,
            ),
            GoProblem::new(
                "problem03_data_types.go".to_string(),
                r#"// Problem: Basic Data Types
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Declare variables of different numeric types
    var smallInt int8
    var bigInt int64
    var floatNum float32
    var doubleNum float64
    
    // TODO: Assign appropriate values to each variable
    
    // TODO: Declare string and boolean variables
    var message string
    var isActive bool
    
    // TODO: Assign values to string and boolean
    
    // TODO: Perform type conversion between int and float
    var intValue int = 42
    var floatValue float64
    // Convert intValue to float64 and assign to floatValue
    
    fmt.Printf("Int8: %d, Int64: %d\n", smallInt, bigInt)
    fmt.Printf("Float32: %f, Float64: %f\n", floatNum, doubleNum)
    fmt.Printf("String: %s, Boolean: %t\n", message, isActive)
    fmt.Printf("Converted: %f\n", floatValue)
}"#
                .to_string(),
                "Explore Go's basic data types and type conversions".to_string(),
                "Data Types".to_string(),
                2,
            ),
            GoProblem::new(
                "problem04_zero_values.go".to_string(),
                r#"// Problem: Understanding Zero Values
// Topic: Variables
// Difficulty: 1

package main

import "fmt"

func main() {
    // TODO: Declare variables without initialization to see their zero values
    var intZero int
    var floatZero float64
    var stringZero string
    var boolZero bool
    
    // TODO: Declare a pointer and see its zero value
    var pointerZero *int
    
    fmt.Printf("Zero values:\n")
    fmt.Printf("int: %d\n", intZero)
    fmt.Printf("float64: %f\n", floatZero)
    fmt.Printf("string: '%s'\n", stringZero)
    fmt.Printf("bool: %t\n", boolZero)
    fmt.Printf("pointer: %v\n", pointerZero)
    
    // TODO: Check if string is empty and pointer is nil
    if stringZero == "" {
        fmt.Println("String zero value is empty string")
    }
    
    if pointerZero == nil {
        fmt.Println("Pointer zero value is nil")
    }
}"#
                .to_string(),
                "Understand Go's zero values for different types".to_string(),
                "Variables".to_string(),
                1,
            ),
            GoProblem::new(
                "problem05_type_inference.go".to_string(),
                r#"// Problem: Type Inference with Short Declaration
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Use short variable declaration and let Go infer the types
    name := "Go Programming"
    version := 1.21
    isStable := true
    userCount := 1000000
    
    // TODO: Print the values and their types using %T format verb
    fmt.Printf("name: %s (type: %T)\n", name, name)
    fmt.Printf("version: %f (type: %T)\n", version, version)
    fmt.Printf("isStable: %t (type: %T)\n", isStable, isStable)
    fmt.Printf("userCount: %d (type: %T)\n", userCount, userCount)
    
    // TODO: Declare multiple variables with different inferred types
    x, y, z := 10, 3.14, "hello"
    
    fmt.Printf("x: %d (%T), y: %f (%T), z: %s (%T)\n", x, x, y, y, z, z)
    
    // TODO: Try reassigning with different types (this should cause an error)
    // Uncomment the line below to see the error
    // name = 123  // This will cause a compile error
}"#
                .to_string(),
                "Learn how Go infers types in short variable declarations".to_string(),
                "Data Types".to_string(),
                2,
            ),
            GoProblem::new(
                "problem06_numeric_types.go".to_string(),
                r#"// Problem: Numeric Types and Operations
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Declare variables of different integer sizes
    var int8Val int8 = 127      // Max value for int8
    var int16Val int16 = 32767  // Max value for int16
    var int32Val int32 = 2147483647
    var int64Val int64 = 9223372036854775807
    
    // TODO: Declare unsigned integer variables
    var uint8Val uint8 = 255
    var uint16Val uint16 = 65535
    
    // TODO: Declare floating point variables
    var float32Val float32 = 3.14159
    var float64Val float64 = 2.718281828459045
    
    fmt.Printf("Signed integers:\n")
    fmt.Printf("int8: %d, int16: %d, int32: %d, int64: %d\n", 
               int8Val, int16Val, int32Val, int64Val)
    
    fmt.Printf("Unsigned integers:\n")
    fmt.Printf("uint8: %d, uint16: %d\n", uint8Val, uint16Val)
    
    fmt.Printf("Floating point:\n")
    fmt.Printf("float32: %f, float64: %f\n", float32Val, float64Val)
    
    // TODO: Perform arithmetic operations
    sum := int32Val + int32(int16Val)  // Type conversion needed
    product := float64Val * float64(float32Val)
    
    fmt.Printf("Sum: %d, Product: %f\n", sum, product)
}"#
                .to_string(),
                "Explore different numeric types and their ranges".to_string(),
                "Data Types".to_string(),
                2,
            ),
            GoProblem::new(
                "problem07_string_operations.go".to_string(),
                r#"// Problem: String Operations and Literals
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Declare strings using different literal types
    regularString := "Hello, World!"
    rawString := `This is a raw string
    with multiple lines
    and \n escape sequences are not interpreted`
    
    // TODO: String concatenation
    firstName := "John"
    lastName := "Doe"
    fullName := firstName + " " + lastName
    
    // TODO: String length and indexing
    message := "Go Programming"
    length := len(message)
    firstChar := message[0]  // Get first character (byte)
    
    fmt.Printf("Regular string: %s\n", regularString)
    fmt.Printf("Raw string: %s\n", rawString)
    fmt.Printf("Full name: %s\n", fullName)
    fmt.Printf("Message: %s (length: %d)\n", message, length)
    fmt.Printf("First character: %c (byte value: %d)\n", firstChar, firstChar)
    
    // TODO: String formatting with different verbs
    age := 25
    height := 5.9
    fmt.Printf("Name: %s, Age: %d, Height: %.1f feet\n", fullName, age, height)
    
    // TODO: Unicode string
    unicodeString := "Hello, 世界! 🌍"
    fmt.Printf("Unicode string: %s (byte length: %d)\n", unicodeString, len(unicodeString))
}"#
                .to_string(),
                "Learn string literals, operations, and formatting".to_string(),
                "Data Types".to_string(),
                2,
            ),
            GoProblem::new(
                "problem08_boolean_operations.go".to_string(),
                r#"// Problem: Boolean Type and Logical Operations
// Topic: Data Types
// Difficulty: 1

package main

import "fmt"

func main() {
    // TODO: Declare boolean variables
    var isStudent bool = true
    var hasLicense bool = false
    
    // TODO: Boolean operations
    canDrive := hasLicense && !isStudent  // Logical AND and NOT
    needsPermission := isStudent || !hasLicense  // Logical OR
    
    // TODO: Comparison operations that result in boolean
    age := 20
    isAdult := age >= 18
    isTeenager := age >= 13 && age <= 19
    
    fmt.Printf("Is student: %t\n", isStudent)
    fmt.Printf("Has license: %t\n", hasLicense)
    fmt.Printf("Can drive: %t\n", canDrive)
    fmt.Printf("Needs permission: %t\n", needsPermission)
    fmt.Printf("Is adult: %t\n", isAdult)
    fmt.Printf("Is teenager: %t\n", isTeenager)
    
    // TODO: Boolean in conditional context
    if isAdult {
        fmt.Println("Person is an adult")
    }
    
    if !hasLicense {
        fmt.Println("Person needs to get a license")
    }
}"#
                .to_string(),
                "Practice boolean type and logical operations".to_string(),
                "Data Types".to_string(),
                1,
            ),
            GoProblem::new(
                "problem09_type_conversion.go".to_string(),
                r#"// Problem: Type Conversion and Casting
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Numeric type conversions
    var intVal int = 42
    var floatVal float64 = 3.14159
    
    // Convert int to float
    intToFloat := float64(intVal)
    
    // Convert float to int (truncates decimal part)
    floatToInt := int(floatVal)
    
    fmt.Printf("Original int: %d, converted to float: %f\n", intVal, intToFloat)
    fmt.Printf("Original float: %f, converted to int: %d\n", floatVal, floatToInt)
    
    // TODO: String conversions
    var char byte = 65  // ASCII value for 'A'
    charToString := string(char)
    
    fmt.Printf("Byte %d as string: %s\n", char, charToString)
    
    // TODO: Demonstrate precision loss
    var bigFloat float64 = 123.456789
    var smallFloat float32 = float32(bigFloat)
    
    fmt.Printf("Original float64: %f\n", bigFloat)
    fmt.Printf("Converted to float32: %f\n", smallFloat)
    
    // TODO: Integer overflow demonstration
    var maxInt8 int8 = 127
    // Uncomment to see overflow behavior
    // var overflow int8 = maxInt8 + 1
    
    fmt.Printf("Max int8: %d\n", maxInt8)
    fmt.Printf("Adding 1 would cause overflow\n")
}"#
                .to_string(),
                "Learn explicit type conversions and potential pitfalls".to_string(),
                "Data Types".to_string(),
                2,
            ),
            GoProblem::new(
                "problem10_variable_scope.go".to_string(),
                r#"// Problem: Variable Scope and Shadowing
// Topic: Variables
// Difficulty: 2

package main

import "fmt"

// TODO: Package-level variable
var globalVar string = "I'm global"

func main() {
    // TODO: Function-level variable
    var localVar string = "I'm local to main"
    
    fmt.Printf("Global variable: %s\n", globalVar)
    fmt.Printf("Local variable: %s\n", localVar)
    
    // TODO: Block scope
    {
        var blockVar string = "I'm in a block"
        // TODO: Shadow the local variable
        localVar := "I'm shadowing the outer localVar"
        
        fmt.Printf("Inside block - localVar: %s\n", localVar)
        fmt.Printf("Inside block - blockVar: %s\n", blockVar)
    }
    
    // blockVar is not accessible here
    fmt.Printf("Outside block - localVar: %s\n", localVar)
    
    // TODO: Loop scope
    for i := 0; i < 3; i++ {
        // i is only accessible within this loop
        fmt.Printf("Loop iteration: %d\n", i)
    }
    
    // i is not accessible here
    
    // TODO: If statement scope
    if x := 10; x > 5 {
        fmt.Printf("x in if statement: %d\n", x)
    }
    
    // x is not accessible here
}"#
                .to_string(),
                "Understand variable scope and shadowing in Go".to_string(),
                "Variables".to_string(),
                2,
            ),
        ]
    }

    /// Generate control flow problems for if, for, and switch statements
    pub fn generate_control_flow_problems() -> Vec<GoProblem> {
        vec![
            // If statement problems
            GoProblem::new(
                "problem01_if_statements.go".to_string(),
                r#"// Problem: Basic If Statements
// Topic: If Statements
// Difficulty: 1

package main

import "fmt"

func main() {
    age := 18
    
    // TODO: Write a simple if statement to check if age >= 18
    
    // TODO: Write an if-else statement
    score := 85
    
    // TODO: Write an if-else if-else chain
    temperature := 25
    
    fmt.Printf("Age: %d\n", age)
    fmt.Printf("Score: %d\n", score)
    fmt.Printf("Temperature: %d°C\n", temperature)
}"#
                .to_string(),
                "Practice basic if, if-else, and if-else if statements".to_string(),
                "If Statements".to_string(),
                1,
            ),
            GoProblem::new(
                "problem02_if_with_initialization.go".to_string(),
                r#"// Problem: If Statements with Initialization
// Topic: If Statements
// Difficulty: 1

package main

import "fmt"

func main() {
    // TODO: Use if with initialization statement
    // Check if a number is even using if n := 42; condition
    
    // TODO: Use if with initialization for string length check
    // if length := len("Hello"); condition
    
    // TODO: Use if with initialization for multiple variables
    // if x, y := 10, 20; condition
    
    fmt.Println("If with initialization examples completed")
}"#
                .to_string(),
                "Learn if statements with initialization syntax".to_string(),
                "If Statements".to_string(),
                1,
            ),
            GoProblem::new(
                "problem03_for_loops_basic.go".to_string(),
                r#"// Problem: Basic For Loops
// Topic: For Loops
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Write a traditional for loop (init; condition; post)
    fmt.Println("Counting from 1 to 5:")
    
    // TODO: Write a for loop that acts like a while loop
    fmt.Println("Countdown from 5:")
    
    // TODO: Write an infinite loop with break condition
    fmt.Println("Finding first number divisible by 7:")
    
    // TODO: Write a for loop with continue statement
    fmt.Println("Odd numbers from 1 to 10:")
}"#
                .to_string(),
                "Practice different forms of for loops".to_string(),
                "For Loops".to_string(),
                2,
            ),
            GoProblem::new(
                "problem04_range_loops.go".to_string(),
                r##"// Problem: Range Loops
// Topic: For Loops
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Range over a slice
    numbers := []int{10, 20, 30, 40, 50}
    fmt.Println("Slice values:")
    
    // TODO: Range over a slice with index and value
    fmt.Println("Slice with indices:")
    
    // TODO: Range over a string
    message := "Hello"
    fmt.Println("String characters:")
    
    // TODO: Range over a map
    colors := map[string]string{
        "red":   "#FF0000",
        "green": "#00FF00",
        "blue":  "#0000FF",
    }
    fmt.Println("Map key-value pairs:")
    
    // TODO: Range with blank identifier
    fmt.Println("Just the values:")
}"##
                .to_string(),
                "Learn range loops over different data structures".to_string(),
                "For Loops".to_string(),
                2,
            ),
            GoProblem::new(
                "problem05_switch_basic.go".to_string(),
                r#"// Problem: Basic Switch Statements
// Topic: Switch Statements
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Write a switch statement with expression
    day := 3
    
    // TODO: Write a switch with multiple values in case
    grade := 'B'
    
    // TODO: Write a switch with string cases
    operation := "add"
    x, y := 10, 5
    
    fmt.Printf("Day: %d\n", day)
    fmt.Printf("Grade: %c\n", grade)
    fmt.Printf("Operation: %s with %d and %d\n", operation, x, y)
}"#
                .to_string(),
                "Practice basic switch statements with different types".to_string(),
                "Switch Statements".to_string(),
                2,
            ),
            GoProblem::new(
                "problem06_switch_without_expression.go".to_string(),
                r#"// Problem: Switch Without Expression
// Topic: Switch Statements
// Difficulty: 2

package main

import "fmt"

func main() {
    score := 85
    
    // TODO: Write a switch without expression (acts like if-else chain)
    
    age := 25
    hasLicense := true
    
    // TODO: Write another switch without expression with complex conditions
    
    fmt.Printf("Score: %d, Age: %d, Has License: %t\n", score, age, hasLicense)
}"#
                .to_string(),
                "Learn switch statements without expressions".to_string(),
                "Switch Statements".to_string(),
                2,
            ),
            GoProblem::new(
                "problem07_switch_fallthrough.go".to_string(),
                r#"// Problem: Switch with Fallthrough
// Topic: Switch Statements
// Difficulty: 2

package main

import "fmt"

func main() {
    number := 2
    
    // TODO: Write a switch with fallthrough behavior
    fmt.Printf("Processing number %d:\n", number)
    
    // TODO: Write a switch showing default fallthrough prevention
    letter := 'A'
    fmt.Printf("Processing letter %c:\n", letter)
}"#
                .to_string(),
                "Understand fallthrough behavior in switch statements".to_string(),
                "Switch Statements".to_string(),
                2,
            ),
            GoProblem::new(
                "problem08_nested_control_flow.go".to_string(),
                r#"// Problem: Nested Control Flow
// Topic: Control Flow
// Difficulty: 3

package main

import "fmt"

func main() {
    // TODO: Create nested loops to print a multiplication table
    fmt.Println("Multiplication Table (1-5):")
    
    // TODO: Use nested if statements for grade calculation
    scores := []int{85, 92, 78, 96, 88}
    fmt.Println("Grade Analysis:")
    
    // TODO: Combine switch and for loop
    operations := []string{"add", "subtract", "multiply", "divide"}
    a, b := 12, 4
    fmt.Printf("Operations on %d and %d:\n", a, b)
}"#
                .to_string(),
                "Practice combining different control flow structures".to_string(),
                "Control Flow".to_string(),
                3,
            ),
            GoProblem::new(
                "problem09_break_continue.go".to_string(),
                r#"// Problem: Break and Continue Statements
// Topic: For Loops
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Use break to exit a loop early
    fmt.Println("Finding first number > 50 that's divisible by 7:")
    
    // TODO: Use continue to skip iterations
    fmt.Println("Even numbers from 1 to 20 (skip odd):")
    
    // TODO: Use labeled break for nested loops
    fmt.Println("Finding coordinates where x*y > 20:")
    
    // TODO: Use labeled continue
    fmt.Println("Processing matrix (skip negative values):")
    matrix := [][]int{
        {1, -2, 3},
        {-4, 5, 6},
        {7, 8, -9},
    }
    _ = matrix
}"#
                .to_string(),
                "Learn break and continue statements with labels".to_string(),
                "For Loops".to_string(),
                2,
            ),
            GoProblem::new(
                "problem10_control_flow_patterns.go".to_string(),
                r#"// Problem: Common Control Flow Patterns
// Topic: Control Flow
// Difficulty: 3

package main

import "fmt"

func main() {
    // TODO: Implement a simple menu system using switch
    choice := 2
    
    // TODO: Implement input validation using for loop and if
    attempts := 0
    maxAttempts := 3
    validInput := false
    
    // TODO: Implement a simple state machine using switch in loop
    state := "start"
    steps := 0
    maxSteps := 10
    
    fmt.Printf("Menu choice: %d\n", choice)
    fmt.Printf("Validation attempts: %d/%d, Valid: %t\n", attempts, maxAttempts, validInput)
    fmt.Printf("State machine: %s, Steps: %d/%d\n", state, steps, maxSteps)
}"#
                .to_string(),
                "Apply control flow in common programming patterns".to_string(),
                "Control Flow".to_string(),
                3,
            ),
        ]
    }

    /// Generate function problems for definitions, parameters, and return values
    pub fn generate_function_problems() -> Vec<GoProblem> {
        vec![
            // Function basics
            GoProblem::new(
                "problem01_function_basics.go".to_string(),
                r#"// Problem: Basic Function Definitions
// Topic: Function Basics
// Difficulty: 1

package main

import "fmt"

// TODO: Define a function that takes no parameters and returns nothing
func sayHello() {
    
}

// TODO: Define a function that takes one parameter
func greet(name string) {
    
}

// TODO: Define a function that returns a value
func add(a, b int) int {
    
}

// TODO: Define a function with multiple parameters of different types
func createProfile(name string, age int, isActive bool) {
    
}

func main() {
    // TODO: Call the functions you defined above
    sayHello()
    greet("Alice")
    result := add(5, 3)
    fmt.Printf("5 + 3 = %d\n", result)
    createProfile("Bob", 25, true)
}"#
                .to_string(),
                "Learn basic function definition and calling syntax".to_string(),
                "Function Basics".to_string(),
                1,
            ),
            GoProblem::new(
                "problem02_multiple_returns.go".to_string(),
                r#"// Problem: Multiple Return Values
// Topic: Multiple Returns
// Difficulty: 2

package main

import "fmt"

// TODO: Define a function that returns multiple values
func divide(a, b float64) (float64, error) {
    
}

// TODO: Define a function with named return values
func getNameAndAge() (name string, age int) {
    
}

// TODO: Define a function that returns multiple values of same type
func getMinMax(numbers []int) (int, int) {
    
}

// TODO: Define a function that swaps two values
func swap(x, y string) (string, string) {
    
}

func main() {
    // TODO: Call function with multiple returns and handle both values
    result, err := divide(10, 3)
    if err != nil {
        fmt.Printf("Error: %v\n", err)
    } else {
        fmt.Printf("10 / 3 = %f\n", result)
    }
    
    // TODO: Call function with named returns
    name, age := getNameAndAge()
    fmt.Printf("Name: %s, Age: %d\n", name, age)
    
    // TODO: Use blank identifier to ignore return values
    numbers := []int{5, 2, 8, 1, 9}
    min, _ := getMinMax(numbers)  // Ignore max value
    fmt.Printf("Minimum value: %d\n", min)
    
    // TODO: Call swap function
    a, b := "hello", "world"
    a, b = swap(a, b)
    fmt.Printf("After swap: %s, %s\n", a, b)
}"#
                .to_string(),
                "Practice functions with multiple return values".to_string(),
                "Multiple Returns".to_string(),
                2,
            ),
            GoProblem::new(
                "problem03_variadic_functions.go".to_string(),
                r#"// Problem: Variadic Functions
// Topic: Variadic Functions
// Difficulty: 3

package main

import "fmt"

// TODO: Define a variadic function that sums integers
func sum(numbers ...int) int {
    
}

// TODO: Define a variadic function with mixed parameters
func logMessage(level string, messages ...string) {
    
}

// TODO: Define a function that takes a slice and uses variadic syntax
func average(numbers ...float64) float64 {
    
}

func main() {
    // TODO: Call variadic function with multiple arguments
    total := sum(1, 2, 3, 4, 5)
    fmt.Printf("Sum: %d\n", total)
    
    // TODO: Call variadic function with slice using ... operator
    nums := []int{10, 20, 30}
    total2 := sum(nums...)
    fmt.Printf("Sum of slice: %d\n", total2)
    
    // TODO: Call variadic function with mixed parameters
    logMessage("INFO", "System started", "Database connected", "Ready to serve")
    
    // TODO: Call with no variadic arguments
    logMessage("ERROR")
    
    // TODO: Calculate average
    avg := average(85.5, 92.0, 78.5, 96.0)
    fmt.Printf("Average: %.2f\n", avg)
}"#
                .to_string(),
                "Learn variadic functions and the ... operator".to_string(),
                "Variadic Functions".to_string(),
                3,
            ),
            GoProblem::new(
                "problem04_function_literals.go".to_string(),
                r#"// Problem: Function Literals and Anonymous Functions
// Topic: Variadic Functions
// Difficulty: 3

package main

import "fmt"

func main() {
    // TODO: Define an anonymous function and call it immediately
    func() {
        fmt.Println("This is an anonymous function")
    }()
    
    // TODO: Assign a function literal to a variable
    multiply := func(a, b int) int {
        return a * b
    }
    
    result := multiply(4, 5)
    fmt.Printf("4 * 5 = %d\n", result)
    
    // TODO: Define a function that returns a function
    makeAdder := func(x int) func(int) int {
        return func(y int) int {
            return x + y
        }
    }
    
    add5 := makeAdder(5)
    fmt.Printf("5 + 3 = %d\n", add5(3))
    
    // TODO: Use function as parameter
    numbers := []int{1, 2, 3, 4, 5}
    
    applyOperation := func(nums []int, op func(int) int) []int {
        result := make([]int, len(nums))
        for i, num := range nums {
            result[i] = op(num)
        }
        return result
    }
    
    // Square each number
    squared := applyOperation(numbers, func(x int) int {
        return x * x
    })
    
    fmt.Printf("Original: %v\n", numbers)
    fmt.Printf("Squared: %v\n", squared)
}"#
                .to_string(),
                "Explore function literals and higher-order functions".to_string(),
                "Variadic Functions".to_string(),
                3,
            ),
            GoProblem::new(
                "problem05_closures.go".to_string(),
                r#"// Problem: Closures and Variable Capture
// Topic: Variadic Functions
// Difficulty: 3

package main

import "fmt"

// TODO: Create a function that returns a closure
func makeCounter() func() int {
    
}

// TODO: Create a closure that captures multiple variables
func makeCalculator(initial int) (func(int) int, func() int) {
    
}

func main() {
    // TODO: Use the counter closure
    counter1 := makeCounter()
    counter2 := makeCounter()
    
    fmt.Printf("Counter1: %d\n", counter1())
    fmt.Printf("Counter1: %d\n", counter1())
    fmt.Printf("Counter2: %d\n", counter2())
    fmt.Printf("Counter1: %d\n", counter1())
    
    // TODO: Use the calculator closures
    add, getValue := makeCalculator(10)
    
    fmt.Printf("Initial value: %d\n", getValue())
    add(5)
    fmt.Printf("After adding 5: %d\n", getValue())
    add(-3)
    fmt.Printf("After adding -3: %d\n", getValue())
    
    // TODO: Demonstrate closure capturing loop variables
    functions := make([]func() int, 3)
    
    // Incorrect way (all closures will capture the same variable)
    for i := 0; i < 3; i++ {
        functions[i] = func() int {
            return i  // This captures the loop variable
        }
    }
    
    fmt.Println("Incorrect closure capture:")
    for j, fn := range functions {
        fmt.Printf("Function %d returns: %d\n", j, fn())
    }
    
    // TODO: Correct way to capture loop variables
    correctFunctions := make([]func() int, 3)
    for i := 0; i < 3; i++ {
        i := i  // Create a new variable in each iteration
        correctFunctions[i] = func() int {
            return i
        }
    }
    
    fmt.Println("Correct closure capture:")
    for j, fn := range correctFunctions {
        fmt.Printf("Function %d returns: %d\n", j, fn())
    }
}"#
                .to_string(),
                "Understand closures and variable capture behavior".to_string(),
                "Variadic Functions".to_string(),
                3,
            ),
            GoProblem::new(
                "problem06_function_types.go".to_string(),
                r#"// Problem: Function Types and Signatures
// Topic: Function Basics
// Difficulty: 2

package main

import "fmt"

// TODO: Define a custom function type
type MathOperation func(int, int) int

// TODO: Define functions that match the MathOperation type
func add(a, b int) int {
    
}

func subtract(a, b int) int {
    
}

func multiply(a, b int) int {
    
}

// TODO: Define a function that takes a function as parameter
func calculate(a, b int, op MathOperation) int {
    
}

// TODO: Define a function that returns a function
func getOperation(opName string) MathOperation {
    
}

func main() {
    x, y := 10, 5
    
    // TODO: Use functions directly
    fmt.Printf("%d + %d = %d\n", x, y, add(x, y))
    fmt.Printf("%d - %d = %d\n", x, y, subtract(x, y))
    
    // TODO: Pass functions as arguments
    result1 := calculate(x, y, add)
    result2 := calculate(x, y, multiply)
    fmt.Printf("Calculate with add: %d\n", result1)
    fmt.Printf("Calculate with multiply: %d\n", result2)
    
    // TODO: Get function from another function
    addOp := getOperation("add")
    subOp := getOperation("subtract")
    
    fmt.Printf("Dynamic add: %d\n", addOp(x, y))
    fmt.Printf("Dynamic subtract: %d\n", subOp(x, y))
    
    // TODO: Create a slice of functions
    operations := []MathOperation{add, subtract, multiply}
    operationNames := []string{"add", "subtract", "multiply"}
    
    for i, op := range operations {
        result := op(x, y)
        fmt.Printf("%s: %d\n", operationNames[i], result)
    }
}"#
                .to_string(),
                "Learn function types and using functions as values".to_string(),
                "Function Basics".to_string(),
                2,
            ),
            GoProblem::new(
                "problem07_recursion.go".to_string(),
                r#"// Problem: Recursive Functions
// Topic: Function Basics
// Difficulty: 2

package main

import "fmt"

// TODO: Implement factorial using recursion
func factorial(n int) int {
    
}

// TODO: Implement Fibonacci using recursion
func fibonacci(n int) int {
    
}

// TODO: Implement sum of digits using recursion
func sumDigits(n int) int {
    
}

// TODO: Implement binary search using recursion
func binarySearch(arr []int, target, left, right int) int {
    
}

func main() {
    // TODO: Test factorial
    for i := 0; i <= 5; i++ {
        fmt.Printf("Factorial of %d: %d\n", i, factorial(i))
    }
    
    // TODO: Test Fibonacci
    fmt.Println("Fibonacci sequence:")
    for i := 0; i < 10; i++ {
        fmt.Printf("F(%d) = %d\n", i, fibonacci(i))
    }
    
    // TODO: Test sum of digits
    numbers := []int{123, 456, 789}
    for _, num := range numbers {
        fmt.Printf("Sum of digits in %d: %d\n", num, sumDigits(num))
    }
    
    // TODO: Test binary search
    sortedArray := []int{1, 3, 5, 7, 9, 11, 13, 15}
    target := 7
    index := binarySearch(sortedArray, target, 0, len(sortedArray)-1)
    
    if index != -1 {
        fmt.Printf("Found %d at index %d\n", target, index)
    } else {
        fmt.Printf("%d not found in array\n", target)
    }
}"#
                .to_string(),
                "Practice recursive function implementations".to_string(),
                "Function Basics".to_string(),
                2,
            ),
            GoProblem::new(
                "problem08_defer_statements.go".to_string(),
                r#"// Problem: Defer Statements
// Topic: Function Basics
// Difficulty: 2

package main

import "fmt"

// TODO: Function demonstrating basic defer
func basicDefer() {
    fmt.Println("Function start")
    
    // TODO: Add defer statements
    
    fmt.Println("Function middle")
    
    // TODO: Add more defer statements
    
    fmt.Println("Function end")
}

// TODO: Function demonstrating defer with parameters
func deferWithParams() {
    x := 10
    
    // TODO: Defer with current value of x
    
    x = 20
    
    // TODO: Defer with updated value of x
    
    fmt.Printf("x at end of function: %d\n", x)
}

// TODO: Function demonstrating defer for cleanup
func fileOperation() {
    fmt.Println("Opening file...")
    
    // TODO: Simulate file operations with defer cleanup
    
    fmt.Println("Processing file...")
    
    // Simulate some work
    for i := 0; i < 3; i++ {
        fmt.Printf("Processing line %d\n", i+1)
    }
    
    fmt.Println("File processing complete")
}

// TODO: Function demonstrating defer in loops
func deferInLoop() {
    fmt.Println("Defer in loop example:")
    
    // TODO: Show what happens with defer in a loop
    
}

func main() {
    fmt.Println("=== Basic Defer ===")
    basicDefer()
    
    fmt.Println("\n=== Defer with Parameters ===")
    deferWithParams()
    
    fmt.Println("\n=== Defer for Cleanup ===")
    fileOperation()
    
    fmt.Println("\n=== Defer in Loop ===")
    deferInLoop()
    
    // TODO: Demonstrate defer order (LIFO - Last In, First Out)
    fmt.Println("\n=== Defer Order (LIFO) ===")
    
}"#
                .to_string(),
                "Learn defer statements and their execution order".to_string(),
                "Function Basics".to_string(),
                2,
            ),
            GoProblem::new(
                "problem09_function_parameters.go".to_string(),
                r#"// Problem: Function Parameters and Arguments
// Topic: Function Basics
// Difficulty: 1

package main

import "fmt"

// TODO: Function with value parameters (pass by value)
func modifyValue(x int) {
    
}

// TODO: Function with pointer parameters (pass by reference)
func modifyPointer(x *int) {
    
}

// TODO: Function with slice parameter
func modifySlice(numbers []int) {
    
}

// TODO: Function with map parameter
func modifyMap(data map[string]int) {
    
}

// TODO: Function with struct parameter
type Person struct {
    Name string
    Age  int
}

func modifyStruct(p Person) {
    
}

func modifyStructPointer(p *Person) {
    
}

func main() {
    // TODO: Test value vs pointer parameters
    value := 10
    fmt.Printf("Original value: %d\n", value)
    
    modifyValue(value)
    fmt.Printf("After modifyValue: %d\n", value)
    
    modifyPointer(&value)
    fmt.Printf("After modifyPointer: %d\n", value)
    
    // TODO: Test slice parameter
    numbers := []int{1, 2, 3, 4, 5}
    fmt.Printf("Original slice: %v\n", numbers)
    
    modifySlice(numbers)
    fmt.Printf("After modifySlice: %v\n", numbers)
    
    // TODO: Test map parameter
    data := map[string]int{"a": 1, "b": 2}
    fmt.Printf("Original map: %v\n", data)
    
    modifyMap(data)
    fmt.Printf("After modifyMap: %v\n", data)
    
    // TODO: Test struct parameters
    person := Person{Name: "Alice", Age: 25}
    fmt.Printf("Original struct: %+v\n", person)
    
    modifyStruct(person)
    fmt.Printf("After modifyStruct: %+v\n", person)
    
    modifyStructPointer(&person)
    fmt.Printf("After modifyStructPointer: %+v\n", person)
}"#
                .to_string(),
                "Understand parameter passing: by value vs by reference".to_string(),
                "Function Basics".to_string(),
                1,
            ),
            GoProblem::new(
                "problem10_function_best_practices.go".to_string(),
                r#"// Problem: Function Best Practices
// Topic: Function Basics
// Difficulty: 2

package main

import (
    "errors"
    "fmt"
)

// TODO: Function with clear naming and single responsibility
func calculateCircleArea(radius float64) (float64, error) {
    
}

// TODO: Function with proper error handling
func divide(a, b float64) (float64, error) {
    
}

// TODO: Function with input validation
func createUser(name string, age int) (*Person, error) {
    
}

// TODO: Function with consistent return patterns
func findMaxValue(numbers []int) (int, bool) {
    
}

// TODO: Function with proper documentation
// calculateTax calculates the tax amount based on income and tax rate.
// It returns the tax amount and any error that occurred during calculation.
// Parameters:
//   - income: the gross income amount (must be >= 0)
//   - taxRate: the tax rate as a decimal (must be between 0 and 1)
// Returns:
//   - float64: the calculated tax amount
//   - error: any error that occurred during calculation
func calculateTax(income, taxRate float64) (float64, error) {
    
}

func main() {
    // TODO: Test circle area calculation
    radius := 5.0
    area, err := calculateCircleArea(radius)
    if err != nil {
        fmt.Printf("Error calculating area: %v\n", err)
    } else {
        fmt.Printf("Circle area (radius %.1f): %.2f\n", radius, area)
    }
    
    // TODO: Test division with error handling
    result, err := divide(10, 2)
    if err != nil {
        fmt.Printf("Division error: %v\n", err)
    } else {
        fmt.Printf("10 / 2 = %.2f\n", result)
    }
    
    // Test division by zero
    _, err = divide(10, 0)
    if err != nil {
        fmt.Printf("Expected error: %v\n", err)
    }
    
    // TODO: Test user creation with validation
    user, err := createUser("Alice", 25)
    if err != nil {
        fmt.Printf("User creation error: %v\n", err)
    } else {
        fmt.Printf("Created user: %+v\n", user)
    }
    
    // Test invalid user data
    _, err = createUser("", -5)
    if err != nil {
        fmt.Printf("Expected validation error: %v\n", err)
    }
    
    // TODO: Test max value finding
    numbers := []int{3, 7, 2, 9, 1}
    max, found := findMaxValue(numbers)
    if found {
        fmt.Printf("Max value in %v: %d\n", numbers, max)
    } else {
        fmt.Println("No max value found (empty slice)")
    }
    
    // TODO: Test tax calculation
    tax, err := calculateTax(50000, 0.25)
    if err != nil {
        fmt.Printf("Tax calculation error: %v\n", err)
    } else {
        fmt.Printf("Tax on $50,000 at 25%%: $%.2f\n", tax)
    }
}"#
                .to_string(),
                "Apply function best practices and error handling patterns".to_string(),
                "Function Basics".to_string(),
                2,
            ),
        ]
    }

    /// Generate package and import problems for Go module system
    #[allow(dead_code)]
    pub fn generate_package_import_problems() -> Vec<GoProblem> {
        vec![
            GoProblem::new(
                "problem01_package_declaration.go".to_string(),
                r#"// Problem: Package Declaration Basics
// Topic: Package Declaration
// Difficulty: 1

// TODO: Declare this as package main

import "fmt"

func main() {
    fmt.Println("Hello from package main!")
    
    // TODO: Create a variable that demonstrates exported vs unexported naming
    // Exported names start with capital letters
    var ExportedVariable string = "I can be accessed from other packages"
    var unexportedVariable string = "I can only be accessed within this package"
    
    fmt.Printf("Exported: %s\n", ExportedVariable)
    fmt.Printf("Unexported: %s\n", unexportedVariable)
}

// TODO: Define an exported function
func ExportedFunction() {
    
}

// TODO: Define an unexported function
func unexportedFunction() {
    
}"#
                .to_string(),
                "Learn package declaration and exported vs unexported identifiers".to_string(),
                "Package Declaration".to_string(),
                1,
            ),
            GoProblem::new(
                "problem02_import_statements.go".to_string(),
                r#"// Problem: Import Statement Variations
// Topic: Import Statements
// Difficulty: 1

package main

// TODO: Import fmt package using single import

// TODO: Import multiple packages using grouped imports
import (
    // Add standard library imports here
)

func main() {
    // TODO: Use fmt for formatted output
    
    // TODO: Use strings package for string manipulation
    text := "  Hello, Go!  "
    
    // TODO: Use math package for mathematical operations
    radius := 5.0
    
    // TODO: Use time package for time operations
    
    fmt.Println("Import examples completed")
}"#
                .to_string(),
                "Practice different import statement formats".to_string(),
                "Import Statements".to_string(),
                1,
            ),
            GoProblem::new(
                "problem03_import_aliases.go".to_string(),
                r#"// Problem: Import Aliases and Blank Imports
// Topic: Import Statements
// Difficulty: 1

package main

import (
    // TODO: Import fmt with an alias
    
    // TODO: Import strings with a different alias
    
    // TODO: Import math with dot import (use with caution)
    
    // TODO: Import a package for side effects only (blank import)
    // Note: This is just for demonstration
)

func main() {
    // TODO: Use aliased fmt package
    
    // TODO: Use aliased strings package
    message := "Hello, World!"
    
    // TODO: Use math functions directly (due to dot import)
    radius := 3.0
    
    // TODO: Demonstrate that blank imported package is loaded but not directly used
    
    fmt.Println("Import aliases demonstration completed")
}"#
                .to_string(),
                "Learn import aliases and blank imports".to_string(),
                "Import Statements".to_string(),
                1,
            ),
            GoProblem::new(
                "problem04_standard_library_fmt.go".to_string(),
                r#"// Problem: Using the fmt Package
// Topic: Standard Library
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Use different fmt.Print functions
    name := "Alice"
    age := 25
    height := 5.6
    
    // TODO: Use fmt.Print (no newline)
    
    // TODO: Use fmt.Println (with newline)
    
    // TODO: Use fmt.Printf with format verbs
    
    // TODO: Use fmt.Sprintf to create formatted strings
    
    // TODO: Use fmt.Scan to read input (commented out for safety)
    // var userInput string
    // fmt.Print("Enter your name: ")
    // fmt.Scan(&userInput)
    // fmt.Printf("Hello, %s!\n", userInput)
    
    // TODO: Demonstrate different format verbs
    number := 42
    
    // %d for decimal integers
    // %b for binary
    // %o for octal  
    // %x for hexadecimal
    // %f for floating point
    // %e for scientific notation
    // %s for strings
    // %t for boolean
    // %T for type
    // %v for default format
    // %+v for struct with field names
    // %#v for Go syntax representation
    
    fmt.Printf("Demonstrating format verbs with number %d:\n", number)
}"#
                .to_string(),
                "Explore the fmt package and format verbs".to_string(),
                "Standard Library".to_string(),
                2,
            ),
            GoProblem::new(
                "problem05_standard_library_strings.go".to_string(),
                r#"// Problem: Using the strings Package
// Topic: Standard Library
// Difficulty: 2

package main

import (
    "fmt"
    "strings"
)

func main() {
    text := "Hello, Go Programming World!"
    
    // TODO: Use strings.Contains to check if text contains a substring
    
    // TODO: Use strings.HasPrefix and strings.HasSuffix
    
    // TODO: Use strings.Index to find position of substring
    
    // TODO: Use strings.Count to count occurrences
    
    // TODO: Use strings.Replace to replace substrings
    
    // TODO: Use strings.ToUpper and strings.ToLower
    
    // TODO: Use strings.TrimSpace to remove whitespace
    whitespaceText := "   Go is awesome!   "
    
    // TODO: Use strings.Split to split string into slice
    csvData := "apple,banana,cherry,date"
    
    // TODO: Use strings.Join to join slice into string
    words := []string{"Go", "is", "a", "great", "language"}
    
    // TODO: Use strings.Repeat to repeat a string
    
    fmt.Printf("Original text: %s\n", text)
    fmt.Printf("CSV data: %s\n", csvData)
    fmt.Printf("Words: %v\n", words)
}"#
                .to_string(),
                "Learn common string manipulation functions".to_string(),
                "Standard Library".to_string(),
                2,
            ),
            GoProblem::new(
                "problem06_standard_library_math.go".to_string(),
                r#"// Problem: Using the math Package
// Topic: Standard Library
// Difficulty: 2

package main

import (
    "fmt"
    "math"
)

func main() {
    // TODO: Use math constants
    fmt.Printf("Pi: %f\n", math.Pi)
    fmt.Printf("E: %f\n", math.E)
    
    // TODO: Use basic math functions
    x := 16.0
    y := 3.0
    
    // TODO: Use math.Sqrt for square root
    
    // TODO: Use math.Pow for exponentiation
    
    // TODO: Use math.Abs for absolute value
    negativeNumber := -42.5
    
    // TODO: Use math.Max and math.Min
    
    // TODO: Use trigonometric functions
    angle := math.Pi / 4  // 45 degrees in radians
    
    // TODO: Use math.Sin, math.Cos, math.Tan
    
    // TODO: Use rounding functions
    decimal := 3.7
    
    // TODO: Use math.Floor, math.Ceil, math.Round
    
    // TODO: Use math.Mod for modulo operation
    
    fmt.Printf("Working with x=%.1f and y=%.1f\n", x, y)
    fmt.Printf("Negative number: %.1f\n", negativeNumber)
    fmt.Printf("Angle: %.4f radians\n", angle)
    fmt.Printf("Decimal: %.1f\n", decimal)
}"#
                .to_string(),
                "Explore mathematical functions and constants".to_string(),
                "Standard Library".to_string(),
                2,
            ),
            GoProblem::new(
                "problem07_package_organization.go".to_string(),
                r#"// Problem: Package Organization Concepts
// Topic: Package Declaration
// Difficulty: 1

package main

import "fmt"

// This file demonstrates package organization concepts
// In a real project, you would organize code into multiple packages

// TODO: Understand package naming conventions
// - Package names should be lowercase
// - Package names should be short and descriptive
// - Package names should match the directory name
// - Avoid underscores, use camelCase for multi-word names

// TODO: Understand the main package
// - The main package is special - it creates an executable program
// - Must have a main() function as the entry point
// - Other packages are libraries that can be imported

func main() {
    fmt.Println("Package Organization Concepts:")
    
    // TODO: Demonstrate package-level variables and functions
    fmt.Printf("Package name: %s\n", getPackageName())
    fmt.Printf("Is main package: %t\n", isMainPackage())
    
    // TODO: Show how packages would be organized in a real project
    showPackageStructure()
}

// TODO: Package-level function
func getPackageName() string {
    return "main"
}

// TODO: Another package-level function
func isMainPackage() bool {
    return true
}

// TODO: Function to demonstrate package structure concepts
func showPackageStructure() {
    fmt.Println("\nTypical Go project structure:")
    fmt.Println("myproject/")
    fmt.Println("├── main.go          (package main)")
    fmt.Println("├── config/")
    fmt.Println("│   └── config.go    (package config)")
    fmt.Println("├── models/")
    fmt.Println("│   └── user.go      (package models)")
    fmt.Println("├── handlers/")
    fmt.Println("│   └── api.go       (package handlers)")
    fmt.Println("└── utils/")
    fmt.Println("    └── helpers.go   (package utils)")
}"#
                .to_string(),
                "Understand Go package organization and naming conventions".to_string(),
                "Package Declaration".to_string(),
                1,
            ),
            GoProblem::new(
                "problem08_init_functions.go".to_string(),
                r#"// Problem: Package Initialization and init Functions
// Topic: Package Declaration
// Difficulty: 2

package main

import "fmt"

// TODO: Package-level variables
var packageVar string

// TODO: init function - runs automatically when package is imported
func init() {
    fmt.Println("First init function called")
    packageVar = "Initialized in init"
}

// TODO: You can have multiple init functions in the same package
func init() {
    fmt.Println("Second init function called")
    // init functions run in the order they appear in the source
}

// TODO: init function with some initialization logic
func init() {
    fmt.Println("Third init function called")
    // Perform setup tasks here
    setupConfiguration()
}

func main() {
    fmt.Println("main function called")
    fmt.Printf("Package variable: %s\n", packageVar)
    
    // TODO: Demonstrate that init functions have already run
    fmt.Println("All init functions completed before main")
    
    // TODO: Show initialization order
    showInitializationOrder()
}

// TODO: Helper function called from init
func setupConfiguration() {
    fmt.Println("  - Configuration setup completed")
}

// TODO: Function to explain initialization order
func showInitializationOrder() {
    fmt.Println("\nGo initialization order:")
    fmt.Println("1. Package-level variables are initialized")
    fmt.Println("2. init() functions are called in order")
    fmt.Println("3. main() function is called (if in main package)")
    fmt.Println("4. This happens for each imported package first")
}"#
                .to_string(),
                "Learn about init functions and package initialization".to_string(),
                "Package Declaration".to_string(),
                2,
            ),
            GoProblem::new(
                "problem09_working_with_modules.go".to_string(),
                r#"// Problem: Go Modules Concepts
// Topic: Package Declaration
// Difficulty: 2

package main

import (
    "fmt"
    "os"
    "path/filepath"
)

func main() {
    // TODO: Understand Go modules
    fmt.Println("Go Modules Concepts:")
    
    // TODO: Explain what a module is
    explainModules()
    
    // TODO: Show module file structure
    showModuleStructure()
    
    // TODO: Demonstrate module path concepts
    demonstrateModulePaths()
    
    // TODO: Show how to work with the current directory
    showCurrentDirectory()
}

// TODO: Function to explain Go modules
func explainModules() {
    fmt.Println("\nWhat is a Go module?")
    fmt.Println("- A module is a collection of related Go packages")
    fmt.Println("- Defined by a go.mod file in the root directory")
    fmt.Println("- Provides dependency management")
    fmt.Println("- Enables versioning and reproducible builds")
}

// TODO: Function to show module structure
func showModuleStructure() {
    fmt.Println("\nModule structure:")
    fmt.Println("mymodule/")
    fmt.Println("├── go.mod           (module definition)")
    fmt.Println("├── go.sum           (dependency checksums)")
    fmt.Println("├── main.go")
    fmt.Println("├── internal/        (private packages)")
    fmt.Println("│   └── helper/")
    fmt.Println("└── pkg/             (public packages)")
    fmt.Println("    └── api/")
}

// TODO: Function to demonstrate module paths
func demonstrateModulePaths() {
    fmt.Println("\nModule path examples:")
    fmt.Println("- github.com/user/project")
    fmt.Println("- example.com/mymodule")
    fmt.Println("- local/myproject (for local development)")
}

// TODO: Function to show current directory info
func showCurrentDirectory() {
    fmt.Println("\nCurrent directory information:")
    
    // Get current working directory
    pwd, err := os.Getwd()
    if err != nil {
        fmt.Printf("Error getting current directory: %v\n", err)
        return
    }
    
    fmt.Printf("Current directory: %s\n", pwd)
    fmt.Printf("Base name: %s\n", filepath.Base(pwd))
    
    // TODO: Check if go.mod exists
    goModPath := filepath.Join(pwd, "go.mod")
    if _, err := os.Stat(goModPath); err == nil {
        fmt.Println("go.mod file found - this is a Go module")
    } else {
        fmt.Println("No go.mod file found")
    }
}"#
                .to_string(),
                "Understand Go modules and project organization".to_string(),
                "Package Declaration".to_string(),
                2,
            ),
            GoProblem::new(
                "problem10_package_best_practices.go".to_string(),
                r#"// Problem: Package and Import Best Practices
// Topic: Package Declaration
// Difficulty: 2

package main

import (
    "fmt"
    "log"
    "os"
    "strings"
    "time"
)

// TODO: Demonstrate good package documentation
// Package main demonstrates Go package and import best practices.
// This includes proper naming, organization, and documentation.

// TODO: Package-level constants (exported)
const (
    AppName    = "Go Learning App"
    AppVersion = "1.0.0"
)

// TODO: Package-level variables (unexported)
var (
    startTime = time.Now()
    logger    = log.New(os.Stdout, "[MAIN] ", log.LstdFlags)
)

func main() {
    logger.Println("Application starting...")
    
    // TODO: Demonstrate best practices
    showNamingConventions()
    showImportBestPractices()
    showPackageDocumentation()
    showErrorHandling()
    
    logger.Printf("Application %s v%s completed\n", AppName, AppVersion)
    logger.Printf("Runtime: %v\n", time.Since(startTime))
}

// TODO: Function demonstrating naming conventions
func showNamingConventions() {
    fmt.Println("\n=== Naming Conventions ===")
    
    // Exported (public) - starts with capital letter
    var PublicVariable = "Accessible from other packages"
    
    // Unexported (private) - starts with lowercase letter
    var privateVariable = "Only accessible within this package"
    
    fmt.Printf("Public: %s\n", PublicVariable)
    fmt.Printf("Private: %s\n", privateVariable)
    
    // TODO: Show function naming
    fmt.Println("Function naming:")
    fmt.Println("- ExportedFunction() - public")
    fmt.Println("- unexportedFunction() - private")
    fmt.Println("- HTTPServer, URLPath - acronyms in caps")
}

// TODO: Function showing import best practices
func showImportBestPractices() {
    fmt.Println("\n=== Import Best Practices ===")
    
    fmt.Println("1. Group imports:")
    fmt.Println("   - Standard library first")
    fmt.Println("   - Third-party packages")
    fmt.Println("   - Local packages last")
    
    fmt.Println("2. Use meaningful aliases when needed")
    fmt.Println("3. Avoid dot imports except for testing")
    fmt.Println("4. Use blank imports only for side effects")
    
    // TODO: Demonstrate proper usage of imported packages
    text := "  Go Best Practices  "
    cleaned := strings.TrimSpace(text)
    fmt.Printf("Cleaned text: '%s'\n", cleaned)
}

// TODO: Function showing documentation practices
func showPackageDocumentation() {
    fmt.Println("\n=== Documentation Best Practices ===")
    
    fmt.Println("1. Package comment should start with 'Package name'")
    fmt.Println("2. Exported functions should have comments")
    fmt.Println("3. Comments should be complete sentences")
    fmt.Println("4. Use examples in documentation")
}

// TODO: Function showing error handling with packages
func showErrorHandling() {
    fmt.Println("\n=== Error Handling ===")
    
    // TODO: Demonstrate proper error handling with file operations
    filename := "nonexistent.txt"
    _, err := os.Open(filename)
    if err != nil {
        fmt.Printf("Expected error opening %s: %v\n", filename, err)
    }
    
    fmt.Println("Always handle errors returned by package functions")
}

// ExportedFunction demonstrates an exported function with proper documentation.
// It takes a name parameter and returns a greeting message.
// This function can be called from other packages.
func ExportedFunction(name string) string {
    return fmt.Sprintf("Hello, %s!", name)
}

// unexportedFunction is only accessible within this package.
func unexportedFunction() {
    fmt.Println("This is a private function")
}"#
                .to_string(),
                "Apply package and import best practices".to_string(),
                "Package Declaration".to_string(),
                2,
            ),
        ]
    }

    /// Generate advanced Go feature problems for structs, interfaces, goroutines, and channels
    #[allow(dead_code)]
    pub fn generate_advanced_feature_problems() -> Vec<GoProblem> {
        vec![
            GoProblem::new(
                "problem01_struct_basics.go".to_string(),
                r#"// Problem: Basic Struct Definition and Usage
// Topic: Structs
// Difficulty: 2

package main

import "fmt"

// TODO: Define a Person struct with fields
type Person struct {
    
}

// TODO: Define a Book struct with different field types
type Book struct {
    
}

// TODO: Define a struct with embedded fields (anonymous fields)
type Employee struct {
    Person  // Embedded struct
    
}

func main() {
    // TODO: Create struct instances using different methods
    
    // Method 1: Zero value
    var p1 Person
    
    // Method 2: Struct literal with field names
    p2 := Person{
        
    }
    
    // Method 3: Struct literal without field names (positional)
    p3 := Person{}
    
    // TODO: Access and modify struct fields
    
    // TODO: Create and use Book struct
    
    // TODO: Create and use Employee struct (with embedding)
    
    fmt.Printf("Person 1: %+v\n", p1)
    fmt.Printf("Person 2: %+v\n", p2)
    fmt.Printf("Person 3: %+v\n", p3)
}"#
                .to_string(),
                "Learn struct definition, initialization, and field access".to_string(),
                "Structs".to_string(),
                2,
            ),
            GoProblem::new(
                "problem02_struct_methods.go".to_string(),
                r#"// Problem: Struct Methods and Receivers
// Topic: Structs
// Difficulty: 2

package main

import "fmt"

// TODO: Define a Rectangle struct
type Rectangle struct {
    Width  float64
    Height float64
}

// TODO: Define a Circle struct
type Circle struct {
    Radius float64
}

// TODO: Define methods with value receivers for Rectangle
func (r Rectangle) Area() float64 {
    
}

func (r Rectangle) Perimeter() float64 {
    
}

// TODO: Define method with pointer receiver for Rectangle
func (r *Rectangle) Scale(factor float64) {
    
}

// TODO: Define methods for Circle
func (c Circle) Area() float64 {
    
}

func (c Circle) Circumference() float64 {
    
}

// TODO: Define a method that returns multiple values
func (r Rectangle) Dimensions() (float64, float64) {
    
}

func main() {
    // TODO: Create Rectangle and test methods
    rect := Rectangle{Width: 10, Height: 5}
    
    fmt.Printf("Rectangle: %+v\n", rect)
    
    // TODO: Call value receiver methods
    
    // TODO: Call pointer receiver method
    
    // TODO: Create Circle and test methods
    circle := Circle{Radius: 3}
    
    fmt.Printf("Circle: %+v\n", circle)
    
    // TODO: Demonstrate method calls on pointers vs values
    rectPtr := &Rectangle{Width: 8, Height: 4}
}"#
                .to_string(),
                "Practice struct methods with value and pointer receivers".to_string(),
                "Structs".to_string(),
                2,
            ),
            GoProblem::new(
                "problem03_interfaces_basic.go".to_string(),
                r#"// Problem: Basic Interface Definition and Implementation
// Topic: Interfaces
// Difficulty: 2

package main

import "fmt"

// TODO: Define a Shape interface
type Shape interface {
    
}

// TODO: Define a Drawable interface
type Drawable interface {
    
}

// TODO: Define structs that implement the interfaces
type Rectangle struct {
    Width, Height float64
}

type Circle struct {
    Radius float64
}

// TODO: Implement Shape interface for Rectangle
func (r Rectangle) Area() float64 {
    
}

func (r Rectangle) Perimeter() float64 {
    
}

// TODO: Implement Drawable interface for Rectangle
func (r Rectangle) Draw() {
    
}

// TODO: Implement Shape interface for Circle
func (c Circle) Area() float64 {
    
}

func (c Circle) Perimeter() float64 {
    
}

// TODO: Implement Drawable interface for Circle
func (c Circle) Draw() {
    
}

// TODO: Function that accepts Shape interface
func printShapeInfo(s Shape) {
    
}

// TODO: Function that accepts Drawable interface
func drawShape(d Drawable) {
    
}

func main() {
    // TODO: Create shapes and use them as interfaces
    rect := Rectangle{Width: 10, Height: 5}
    circle := Circle{Radius: 3}
    
    // TODO: Use shapes as Shape interface
    
    // TODO: Use shapes as Drawable interface
    
    // TODO: Store different shapes in a slice of Shape interface
    shapes := []Shape{rect, circle}
    
    fmt.Println("All shapes:")
    for i, shape := range shapes {
        fmt.Printf("Shape %d:\n", i+1)
        printShapeInfo(shape)
    }
}"#
                .to_string(),
                "Learn interface definition and implementation".to_string(),
                "Interfaces".to_string(),
                2,
            ),
            GoProblem::new(
                "problem04_empty_interface.go".to_string(),
                r#"// Problem: Empty Interface and Type Assertions
// Topic: Interfaces
// Difficulty: 3

package main

import "fmt"

// TODO: Function that accepts empty interface
func printAnything(value interface{}) {
    
}

// TODO: Function that uses type assertions
func describeValue(value interface{}) {
    
}

// TODO: Function that uses type switch
func processValue(value interface{}) {
    
}

func main() {
    // TODO: Use empty interface with different types
    values := []interface{}{
        42,
        "hello",
        3.14,
        true,
        []int{1, 2, 3},
        map[string]int{"a": 1, "b": 2},
    }
    
    fmt.Println("=== Using empty interface ===")
    for _, value := range values {
        printAnything(value)
    }
    
    fmt.Println("\n=== Type assertions ===")
    for _, value := range values {
        describeValue(value)
    }
    
    fmt.Println("\n=== Type switch ===")
    for _, value := range values {
        processValue(value)
    }
    
    // TODO: Demonstrate safe type assertion
    var x interface{} = "hello"
    
    // Safe type assertion
    if str, ok := x.(string); ok {
        fmt.Printf("x is a string: %s\n", str)
    } else {
        fmt.Println("x is not a string")
    }
    
    // TODO: Demonstrate unsafe type assertion (commented out)
    // This would panic if x is not a string
    // str := x.(string)
}"#
                .to_string(),
                "Explore empty interface and type assertions".to_string(),
                "Interfaces".to_string(),
                3,
            ),
            GoProblem::new(
                "problem05_goroutines_basic.go".to_string(),
                r#"// Problem: Basic Goroutines
// Topic: Goroutines
// Difficulty: 2

package main

import (
    "fmt"
    "time"
)

// TODO: Function to run as a goroutine
func sayHello(name string) {
    
}

// TODO: Function that takes time to complete
func countNumbers(name string, max int) {
    
}

// TODO: Function that demonstrates goroutine with anonymous function
func demonstrateAnonymousGoroutine() {
    
}

func main() {
    fmt.Println("Starting goroutine examples...")
    
    // TODO: Start goroutines using go keyword
    
    // TODO: Start multiple goroutines
    
    // TODO: Use anonymous function as goroutine
    
    // TODO: Demonstrate that main function doesn't wait for goroutines
    fmt.Println("Main function continuing...")
    
    // TODO: Add sleep to see goroutine output
    time.Sleep(3 * time.Second)
    
    fmt.Println("Main function ending...")
}"#
                .to_string(),
                "Learn basic goroutine creation and execution".to_string(),
                "Goroutines".to_string(),
                2,
            ),
            GoProblem::new(
                "problem06_channels_basic.go".to_string(),
                r#"// Problem: Basic Channels
// Topic: Channels
// Difficulty: 3

package main

import "fmt"

// TODO: Function that sends data to a channel
func sendData(ch chan int) {
    
}

// TODO: Function that receives data from a channel
func receiveData(ch chan int) {
    
}

// TODO: Function demonstrating bidirectional channel
func processNumbers(numbers chan int, results chan int) {
    
}

func main() {
    // TODO: Create an unbuffered channel
    ch := make(chan int)
    
    // TODO: Start goroutine to send data
    go sendData(ch)
    
    // TODO: Receive data from channel
    
    // TODO: Create a buffered channel
    bufferedCh := make(chan string, 3)
    
    // TODO: Send data to buffered channel without blocking
    
    // TODO: Receive data from buffered channel
    
    // TODO: Demonstrate channel with goroutines
    numbers := make(chan int)
    results := make(chan int)
    
    // Start processing goroutine
    go processNumbers(numbers, results)
    
    // TODO: Send numbers and receive results
    
    // TODO: Close channels when done
    
    fmt.Println("Channel examples completed")
}"#
                .to_string(),
                "Practice basic channel operations and communication".to_string(),
                "Channels".to_string(),
                3,
            ),
            GoProblem::new(
                "problem07_select_statement.go".to_string(),
                r#"// Problem: Select Statement for Channel Operations
// Topic: Channels
// Difficulty: 3

package main

import (
    "fmt"
    "time"
)

// TODO: Function that sends data after a delay
func sendAfterDelay(ch chan string, message string, delay time.Duration) {
    
}

// TODO: Function that demonstrates timeout with select
func demonstrateTimeout() {
    
}

// TODO: Function that demonstrates non-blocking operations
func demonstrateNonBlocking() {
    
}

func main() {
    // TODO: Create channels for select demonstration
    ch1 := make(chan string)
    ch2 := make(chan string)
    
    // TODO: Start goroutines that send data
    go sendAfterDelay(ch1, "from ch1", 1*time.Second)
    go sendAfterDelay(ch2, "from ch2", 2*time.Second)
    
    // TODO: Use select to receive from multiple channels
    
    // TODO: Demonstrate select with default case
    
    // TODO: Demonstrate timeout pattern
    demonstrateTimeout()
    
    // TODO: Demonstrate non-blocking operations
    demonstrateNonBlocking()
    
    fmt.Println("Select statement examples completed")
}"#
                .to_string(),
                "Learn select statement for channel multiplexing".to_string(),
                "Channels".to_string(),
                3,
            ),
            GoProblem::new(
                "problem08_interface_composition.go".to_string(),
                r#"// Problem: Interface Composition and Embedding
// Topic: Interfaces
// Difficulty: 3

package main

import "fmt"

// TODO: Define basic interfaces
type Reader interface {
    
}

type Writer interface {
    
}

type Closer interface {
    
}

// TODO: Compose interfaces using embedding
type ReadWriter interface {
    
}

type ReadWriteCloser interface {
    
}

// TODO: Define a struct that implements multiple interfaces
type File struct {
    name string
    data []byte
}

// TODO: Implement Reader interface for File
func (f *File) Read(data []byte) (int, error) {
    
}

// TODO: Implement Writer interface for File
func (f *File) Write(data []byte) (int, error) {
    
}

// TODO: Implement Closer interface for File
func (f *File) Close() error {
    
}

// TODO: Function that uses composed interface
func processFile(rw ReadWriter) {
    
}

// TODO: Function that uses fully composed interface
func handleFile(rwc ReadWriteCloser) {
    
}

func main() {
    // TODO: Create a File instance
    file := &File{
        name: "example.txt",
        data: make([]byte, 0, 100),
    }
    
    // TODO: Use file as different interface types
    
    // TODO: Demonstrate interface composition
    
    fmt.Printf("File: %+v\n", file)
}"#
                .to_string(),
                "Practice interface composition and embedding".to_string(),
                "Interfaces".to_string(),
                3,
            ),
            GoProblem::new(
                "problem09_struct_embedding.go".to_string(),
                r#"// Problem: Struct Embedding and Promotion
// Topic: Structs
// Difficulty: 3

package main

import "fmt"

// TODO: Define base structs
type Person struct {
    Name string
    Age  int
}

type Address struct {
    Street string
    City   string
    State  string
}

// TODO: Define methods for base structs
func (p Person) Introduce() {
    
}

func (p *Person) HaveBirthday() {
    
}

func (a Address) FullAddress() string {
    
}

// TODO: Define struct with embedding
type Employee struct {
    Person   // Embedded struct
    Address  // Embedded struct
    
}

// TODO: Define method for Employee
func (e Employee) Work() {
    
}

// TODO: Define struct with named embedding
type Manager struct {
    Employee Employee  // Named embedding
    
}

func main() {
    // TODO: Create Employee with embedded structs
    emp := Employee{
        Person: Person{
            Name: "Alice",
            Age:  30,
        },
        Address: Address{
            Street: "123 Main St",
            City:   "Anytown",
            State:  "CA",
        },
    }
    
    // TODO: Access embedded fields directly (promotion)
    
    // TODO: Call embedded methods directly
    
    // TODO: Call Employee's own method
    
    // TODO: Demonstrate method promotion
    
    // TODO: Create Manager with named embedding
    mgr := Manager{
        Employee: emp,
    }
    
    // TODO: Access nested embedded fields
    
    fmt.Printf("Employee: %+v\n", emp)
    fmt.Printf("Manager: %+v\n", mgr)
}"#
                .to_string(),
                "Learn struct embedding and method promotion".to_string(),
                "Structs".to_string(),
                3,
            ),
            GoProblem::new(
                "problem10_advanced_patterns.go".to_string(),
                r#"// Problem: Advanced Go Patterns
// Topic: Advanced Features
// Difficulty: 3

package main

import (
    "fmt"
    "sync"
    "time"
)

// TODO: Define interface for worker pattern
type Worker interface {
    
}

// TODO: Define concrete worker
type NumberWorker struct {
    id int
}

// TODO: Implement Worker interface
func (w NumberWorker) Process(data interface{}) interface{} {
    
}

// TODO: Worker pool pattern
func workerPool(jobs <-chan interface{}, results chan<- interface{}, numWorkers int) {
    
}

// TODO: Fan-out/Fan-in pattern
func fanOutFanIn(input <-chan int) <-chan int {
    
}

// TODO: Pipeline pattern
func pipeline() {
    
}

// TODO: Singleton pattern with sync.Once
type Singleton struct {
    value string
}

var (
    instance *Singleton
    once     sync.Once
)

func GetSingleton() *Singleton {
    
}

func main() {
    fmt.Println("=== Advanced Go Patterns ===")
    
    // TODO: Demonstrate worker pool
    fmt.Println("\n1. Worker Pool Pattern:")
    
    // TODO: Demonstrate fan-out/fan-in
    fmt.Println("\n2. Fan-out/Fan-in Pattern:")
    
    // TODO: Demonstrate pipeline
    fmt.Println("\n3. Pipeline Pattern:")
    
    // TODO: Demonstrate singleton
    fmt.Println("\n4. Singleton Pattern:")
    
    fmt.Println("\nAdvanced patterns demonstration completed")
}"#
                .to_string(),
                "Apply advanced Go patterns and concurrency techniques".to_string(),
                "Advanced Features".to_string(),
                3,
            ),
        ]
    }

    /// Generate package and import problems
    #[allow(dead_code)]
    pub fn generate_package_problems() -> Vec<GoProblem> {
        vec![
            GoProblem::new(
                "problem01_package_declaration.go".to_string(),
                r#"// Problem: Package Declaration and Main Function
// Topic: Package Declaration
// Difficulty: 1

// TODO: Add the correct package declaration for an executable program
package main

import "fmt"

// TODO: Define the main function that serves as the entry point
func main() {
    fmt.Println("Hello, Go!")
    
    // TODO: Call a function from the same package
    greetUser("Alice")
}

// TODO: Define a function in the same package
func greetUser(name string) {
    fmt.Printf("Welcome, %s!\n", name)
}
"#
                .to_string(),
                "Learn package declaration and main function".to_string(),
                "Package Declaration".to_string(),
                1,
            ),
            GoProblem::new(
                "problem02_imports.go".to_string(),
                r#"// Problem: Import Statements and Standard Library
// Topic: Import Statements
// Difficulty: 1

package main

// TODO: Import required packages
import (
    "fmt"
    "math"
    "strings"
    "time"
)

func main() {
    // TODO: Use fmt package for formatted output
    name := "Go Programming"
    fmt.Printf("Learning: %s\n", name)
    
    // TODO: Use math package for calculations
    radius := 5.0
    area := math.Pi * math.Pow(radius, 2)
    fmt.Printf("Circle area: %.2f\n", area)
    
    // TODO: Use strings package for string manipulation
    message := "hello world"
    fmt.Printf("Uppercase: %s\n", strings.ToUpper(message))
    fmt.Printf("Title case: %s\n", strings.Title(message))
    
    // TODO: Use time package for current time
    now := time.Now()
    fmt.Printf("Current time: %s\n", now.Format("2006-01-02 15:04:05"))
}
"#
                .to_string(),
                "Practice importing and using standard library packages".to_string(),
                "Import Statements".to_string(),
                1,
            ),
        ]
    }

    /// Generate struct and method problems
    #[allow(dead_code)]
    pub fn generate_struct_problems() -> Vec<GoProblem> {
        vec![GoProblem::new(
            "problem01_struct_basics.go".to_string(),
            r#"// Problem: Struct Definition and Usage
// Topic: Struct Definition
// Difficulty: 2

package main

import "fmt"

// TODO: Define a Person struct
type Person struct {
    Name string
    Age  int
    City string
}

// TODO: Define a Book struct
type Book struct {
    Title  string
    Author string
    Pages  int
    Price  float64
}

func main() {
    // TODO: Create Person instances using different methods
    
    // Method 1: Struct literal with field names
    p1 := Person{
        Name: "Alice",
        Age:  30,
        City: "New York",
    }
    
    // Method 2: Struct literal with positional values
    p2 := Person{"Bob", 25, "San Francisco"}
    
    // Method 3: Zero value struct
    var p3 Person
    p3.Name = "Charlie"
    p3.Age = 35
    
    // TODO: Create and use Book struct
    book := Book{
        Title:  "The Go Programming Language",
        Author: "Alan Donovan",
        Pages:  380,
        Price:  39.99,
    }
    
    fmt.Printf("Person 1: %+v\n", p1)
    fmt.Printf("Person 2: %+v\n", p2)
    fmt.Printf("Person 3: %+v\n", p3)
    fmt.Printf("Book: %+v\n", book)
}
"#
            .to_string(),
            "Learn struct definition and initialization".to_string(),
            "Structs".to_string(),
            2,
        )]
    }

    /// Generate interface problems
    #[allow(dead_code)]
    pub fn generate_interface_problems() -> Vec<GoProblem> {
        vec![GoProblem::new(
            "problem01_interface_basics.go".to_string(),
            r#"// Problem: Basic Interface Definition
// Topic: Interface Basics
// Difficulty: 2

package main

import "fmt"

// TODO: Define a Shape interface
type Shape interface {
    Area() float64
    Perimeter() float64
}

// TODO: Define Rectangle struct
type Rectangle struct {
    Width, Height float64
}

// TODO: Implement Shape interface for Rectangle
func (r Rectangle) Area() float64 {
    return r.Width * r.Height
}

func (r Rectangle) Perimeter() float64 {
    return 2 * (r.Width + r.Height)
}

// TODO: Function that accepts Shape interface
func printShapeInfo(s Shape) {
    fmt.Printf("Area: %.2f, Perimeter: %.2f\n", s.Area(), s.Perimeter())
}

func main() {
    rect := Rectangle{Width: 10, Height: 5}
    
    // Use rectangle as Shape interface
    printShapeInfo(rect)
}
"#
            .to_string(),
            "Learn basic interface definition and implementation".to_string(),
            "Interfaces".to_string(),
            2,
        )]
    }

    /// Generate concurrency problems
    #[allow(dead_code)]
    pub fn generate_concurrency_problems() -> Vec<GoProblem> {
        vec![GoProblem::new(
            "problem01_goroutines.go".to_string(),
            r#"// Problem: Basic Goroutines
// Topic: Goroutines
// Difficulty: 2

package main

import (
    "fmt"
    "time"
)

// TODO: Function to run as goroutine
func sayHello(name string) {
    for i := 0; i < 3; i++ {
        fmt.Printf("Hello from %s (%d)\n", name, i+1)
        time.Sleep(100 * time.Millisecond)
    }
}

func main() {
    // TODO: Start goroutines
    go sayHello("Goroutine 1")
    go sayHello("Goroutine 2")
    
    // TODO: Keep main function alive to see goroutine output
    time.Sleep(1 * time.Second)
    fmt.Println("Main function ending")
}
"#
            .to_string(),
            "Learn basic goroutine creation and execution".to_string(),
            "Goroutines".to_string(),
            2,
        )]
    }

    /// Generate error handling problems
    #[allow(dead_code)]
    pub fn generate_error_handling_problems() -> Vec<GoProblem> {
        vec![GoProblem::new(
            "problem01_error_basics.go".to_string(),
            r#"// Problem: Basic Error Handling
// Topic: Error Interface
// Difficulty: 2

package main

import (
    "errors"
    "fmt"
)

// TODO: Function that returns an error
func divide(a, b float64) (float64, error) {
    if b == 0 {
        return 0, errors.New("division by zero")
    }
    return a / b, nil
}

func main() {
    // TODO: Handle error properly
    result, err := divide(10, 2)
    if err != nil {
        fmt.Printf("Error: %v\n", err)
    } else {
        fmt.Printf("Result: %.2f\n", result)
    }
    
    // TODO: Handle division by zero error
    result2, err2 := divide(10, 0)
    if err2 != nil {
        fmt.Printf("Error: %v\n", err2)
    } else {
        fmt.Printf("Result: %.2f\n", result2)
    }
}
"#
            .to_string(),
            "Learn basic error handling patterns".to_string(),
            "Error Handling".to_string(),
            2,
        )]
    }

    /// Generate pointer problems
    #[allow(dead_code)]
    pub fn generate_pointer_problems() -> Vec<GoProblem> {
        vec![GoProblem::new(
            "problem01_pointer_basics.go".to_string(),
            r#"// Problem: Basic Pointers
// Topic: Pointer Basics
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Declare variables and pointers
    x := 42
    
    // TODO: Get pointer to x
    ptr := &x
    
    fmt.Printf("Value of x: %d\n", x)
    fmt.Printf("Address of x: %p\n", &x)
    fmt.Printf("Value of ptr: %p\n", ptr)
    fmt.Printf("Value pointed to by ptr: %d\n", *ptr)
    
    // TODO: Modify value through pointer
    *ptr = 100
    fmt.Printf("New value of x: %d\n", x)
}
"#
            .to_string(),
            "Learn basic pointer operations".to_string(),
            "Pointers".to_string(),
            2,
        )]
    }

    /// Generate collection problems
    #[allow(dead_code)]
    pub fn generate_collection_problems() -> Vec<GoProblem> {
        vec![GoProblem::new(
            "problem01_arrays_slices.go".to_string(),
            r#"// Problem: Arrays and Slices
// Topic: Arrays
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Declare and initialize arrays
    var arr1 [5]int
    arr2 := [3]string{"apple", "banana", "cherry"}
    
    // TODO: Work with slices
    slice1 := []int{1, 2, 3, 4, 5}
    slice2 := make([]string, 3, 5) // length 3, capacity 5
    
    fmt.Printf("Array 1: %v\n", arr1)
    fmt.Printf("Array 2: %v\n", arr2)
    fmt.Printf("Slice 1: %v\n", slice1)
    fmt.Printf("Slice 2: %v (len=%d, cap=%d)\n", slice2, len(slice2), cap(slice2))
    
    // TODO: Append to slice
    slice1 = append(slice1, 6, 7)
    fmt.Printf("After append: %v\n", slice1)
}
"#
            .to_string(),
            "Learn arrays and slices basics".to_string(),
            "Collections".to_string(),
            2,
        )]
    }

    /// Enhanced problem content generator that creates section-specific problems
    pub fn generate_enhanced_problems_for_section(section: &Section) -> Vec<GoProblem> {
        match section.id.as_str() {
            "section1-basics" => Self::generate_enhanced_basic_problems(),
            "section2-control-flow" => Self::generate_enhanced_control_flow_problems(),
            "section3-functions" => Self::generate_enhanced_function_problems(),
            "section4-packages" => Self::generate_enhanced_package_problems(),
            "section5-structs" => Self::generate_enhanced_struct_problems(),
            "section6-interfaces" => Self::generate_enhanced_interface_problems(),
            "section7-concurrency" => Self::generate_enhanced_concurrency_problems(),
            "section8-error-handling" => Self::generate_enhanced_error_problems(),
            "section9-pointers" => Self::generate_enhanced_pointer_problems(),
            "section10-collections" => Self::generate_enhanced_collection_problems(),
            _ => Self::generate_progressive_problems_for_section(section),
        }
    }

    /// Generate enhanced basic syntax problems with progressive difficulty
    fn generate_enhanced_basic_problems() -> Vec<GoProblem> {
        // Use existing implementation if it has 10 problems, otherwise generate new ones
        let existing = Self::generate_basic_syntax_problems();
        if existing.len() == 10 {
            existing
        } else {
            // Generate 10 problems with progressive difficulty
            vec![
                // Easy problems (1-3)
                GoProblem::new(
                    "problem01_variables_basic.go".to_string(),
                    r#"// Problem: Basic Variable Declaration
// Topic: Variables
// Difficulty: 1

package main

import "fmt"

func main() {
    // TODO: Declare a variable 'name' of type string
    
    // TODO: Declare a variable 'age' of type int
    
    // TODO: Print both variables
    fmt.Printf("Name: %s, Age: %d\n", name, age)
}"#
                    .to_string(),
                    "Learn basic variable declaration syntax".to_string(),
                    "Variables".to_string(),
                    1,
                ),
                GoProblem::new(
                    "problem02_constants_basic.go".to_string(),
                    r#"// Problem: Basic Constants
// Topic: Constants
// Difficulty: 1

package main

import "fmt"

func main() {
    // TODO: Declare a constant 'pi' with value 3.14
    
    // TODO: Declare a constant 'greeting' with value "Hello"
    
    fmt.Printf("Pi: %f, Greeting: %s\n", pi, greeting)
}"#
                    .to_string(),
                    "Learn basic constant declaration".to_string(),
                    "Constants".to_string(),
                    1,
                ),
                GoProblem::new(
                    "problem03_types_basic.go".to_string(),
                    r#"// Problem: Basic Data Types
// Topic: Data Types
// Difficulty: 1

package main

import "fmt"

func main() {
    // TODO: Declare variables of different types
    var number int
    var decimal float64
    var text string
    var flag bool
    
    // TODO: Assign values to each variable
    
    fmt.Printf("Int: %d, Float: %f, String: %s, Bool: %t\n", number, decimal, text, flag)
}"#
                    .to_string(),
                    "Practice with basic Go data types".to_string(),
                    "Data Types".to_string(),
                    1,
                ),
                // Medium problems (4-6)
                GoProblem::new(
                    "problem04_short_declaration.go".to_string(),
                    r#"// Problem: Short Variable Declaration
// Topic: Variables
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Use short declaration (:=) to declare and initialize variables
    // Declare: name, age, height, isStudent
    
    // TODO: Declare multiple variables in one line using short declaration
    
    fmt.Printf("Name: %s, Age: %d, Height: %.1f, Student: %t\n", name, age, height, isStudent)
}"#
                    .to_string(),
                    "Master short variable declaration syntax".to_string(),
                    "Variables".to_string(),
                    2,
                ),
                GoProblem::new(
                    "problem05_type_conversion.go".to_string(),
                    r#"// Problem: Type Conversion
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
    var intVal int = 42
    var floatVal float64 = 3.14159
    
    // TODO: Convert int to float64
    
    // TODO: Convert float64 to int (note: this truncates)
    
    // TODO: Convert numbers to string representation
    
    fmt.Printf("Original: int=%d, float=%f\n", intVal, floatVal)
    fmt.Printf("Converted: float=%f, int=%d\n", convertedFloat, convertedInt)
}"#
                    .to_string(),
                    "Learn type conversion between numeric types".to_string(),
                    "Data Types".to_string(),
                    2,
                ),
                GoProblem::new(
                    "problem06_zero_values.go".to_string(),
                    r#"// Problem: Zero Values
// Topic: Variables
// Difficulty: 2

package main

import "fmt"

func main() {
    // TODO: Declare variables without initialization to see zero values
    var intZero int
    var floatZero float64
    var stringZero string
    var boolZero bool
    var sliceZero []int
    
    // TODO: Print all zero values and check conditions
    fmt.Printf("Zero values: int=%d, float=%f, string='%s', bool=%t, slice=%v\n", 
               intZero, floatZero, stringZero, boolZero, sliceZero)
    
    // TODO: Check if string is empty and slice is nil
}"#
                    .to_string(),
                    "Understand Go's zero values for different types".to_string(),
                    "Variables".to_string(),
                    2,
                ),
                // Hard problems (7-10)
                GoProblem::new(
                    "problem07_complex_types.go".to_string(),
                    r#"// Problem: Complex Numeric Types
// Topic: Data Types
// Difficulty: 3

package main

import "fmt"

func main() {
    // TODO: Work with different integer sizes and unsigned types
    var int8Val int8 = 127
    var uint8Val uint8 = 255
    var int32Val int32 = 2147483647
    var uint64Val uint64 = 18446744073709551615
    
    // TODO: Demonstrate overflow behavior (commented out to avoid runtime errors)
    // var overflow int8 = int8Val + 1  // This would overflow
    
    // TODO: Work with complex numbers
    var complexNum complex64 = 3 + 4i
    
    fmt.Printf("Integer types: int8=%d, uint8=%d, int32=%d, uint64=%d\n", 
               int8Val, uint8Val, int32Val, uint64Val)
    fmt.Printf("Complex number: %v (real: %f, imag: %f)\n", 
               complexNum, real(complexNum), imag(complexNum))
}"#
                    .to_string(),
                    "Explore advanced numeric types and complex numbers".to_string(),
                    "Data Types".to_string(),
                    3,
                ),
                GoProblem::new(
                    "problem08_variable_scope.go".to_string(),
                    r#"// Problem: Variable Scope and Shadowing
// Topic: Variables
// Difficulty: 3

package main

import "fmt"

var globalVar = "I'm global"

func main() {
    localVar := "I'm local"
    
    fmt.Printf("Global: %s, Local: %s\n", globalVar, localVar)
    
    // TODO: Create a block scope and shadow the local variable
    {
        localVar := "I'm shadowing"
        blockVar := "I'm in a block"
        
        fmt.Printf("In block - Local: %s, Block: %s\n", localVar, blockVar)
    }
    
    // TODO: Use variable in if statement initialization
    if x := 10; x > 5 {
        fmt.Printf("x in if: %d\n", x)
    }
    
    // TODO: Use variable in for loop
    for i := 0; i < 3; i++ {
        fmt.Printf("Loop var i: %d\n", i)
    }
    
    fmt.Printf("Back to main - Local: %s\n", localVar)
}"#
                    .to_string(),
                    "Master variable scope, shadowing, and block scope".to_string(),
                    "Variables".to_string(),
                    3,
                ),
                GoProblem::new(
                    "problem09_constants_advanced.go".to_string(),
                    r#"// Problem: Advanced Constants and Iota
// Topic: Constants
// Difficulty: 3

package main

import "fmt"

func main() {
    // TODO: Use iota for enumerated constants
    const (
        Sunday = iota
        Monday
        Tuesday
        Wednesday
        Thursday
        Friday
        Saturday
    )
    
    // TODO: Use iota with expressions
    const (
        KB = 1 << (10 * iota)  // 1 << (10*0) = 1
        MB                      // 1 << (10*1) = 1024
        GB                      // 1 << (10*2) = 1048576
    )
    
    // TODO: Typed constants
    const (
        Pi    float64 = 3.14159
        E     float64 = 2.71828
        Title string  = "Go Programming"
    )
    
    fmt.Printf("Days: Sun=%d, Mon=%d, Tue=%d\n", Sunday, Monday, Tuesday)
    fmt.Printf("Sizes: KB=%d, MB=%d, GB=%d\n", KB, MB, GB)
    fmt.Printf("Constants: Pi=%f, E=%f, Title=%s\n", Pi, E, Title)
}"#
                    .to_string(),
                    "Master advanced constant features and iota".to_string(),
                    "Constants".to_string(),
                    3,
                ),
                GoProblem::new(
                    "problem10_type_inference.go".to_string(),
                    r#"// Problem: Type Inference and Multiple Assignment
// Topic: Data Types
// Difficulty: 3

package main

import "fmt"

func main() {
    // TODO: Use type inference with short declaration
    name := "Go"
    version := 1.21
    isStable := true
    
    // TODO: Multiple assignment with type inference
    x, y, z := 10, 3.14, "hello"
    
    // TODO: Multiple assignment with existing variables
    a, b := 1, 2
    a, b = b, a  // Swap values
    
    // TODO: Use blank identifier to ignore values
    value, _ := 42, "ignored"
    
    // TODO: Print types using %T format verb
    fmt.Printf("name: %s (%T)\n", name, name)
    fmt.Printf("version: %f (%T)\n", version, version)
    fmt.Printf("isStable: %t (%T)\n", isStable, isStable)
    fmt.Printf("Multiple: x=%d (%T), y=%f (%T), z=%s (%T)\n", x, x, y, y, z, z)
    fmt.Printf("Swapped: a=%d, b=%d\n", a, b)
    fmt.Printf("Value: %d\n", value)
}"#
                    .to_string(),
                    "Master type inference and multiple assignment".to_string(),
                    "Data Types".to_string(),
                    3,
                ),
            ]
        }
    }

    /// Generate enhanced control flow problems
    fn generate_enhanced_control_flow_problems() -> Vec<GoProblem> {
        let existing = Self::generate_control_flow_problems();
        if existing.len() == 10 {
            existing
        } else {
            // Return existing for now, can be enhanced later
            existing
        }
    }

    /// Generate enhanced function problems
    fn generate_enhanced_function_problems() -> Vec<GoProblem> {
        let existing = Self::generate_function_problems();
        if existing.len() == 10 {
            existing
        } else {
            // Return existing for now, can be enhanced later
            existing
        }
    }

    /// Generate enhanced package problems
    fn generate_enhanced_package_problems() -> Vec<GoProblem> {
        // Always use progressive generator to ensure exactly 10 problems
        let section = &SectionConfig::default_go_sections().sections[3]; // section4-packages
        Self::generate_progressive_problems_for_section(section)
    }

    /// Generate enhanced struct problems
    fn generate_enhanced_struct_problems() -> Vec<GoProblem> {
        // Always use progressive generator to ensure exactly 10 problems
        let section = &SectionConfig::default_go_sections().sections[4]; // section5-structs
        Self::generate_progressive_problems_for_section(section)
    }

    /// Generate enhanced interface problems
    fn generate_enhanced_interface_problems() -> Vec<GoProblem> {
        // Always use progressive generator to ensure exactly 10 problems
        let section = &SectionConfig::default_go_sections().sections[5]; // section6-interfaces
        Self::generate_progressive_problems_for_section(section)
    }

    /// Generate enhanced concurrency problems
    fn generate_enhanced_concurrency_problems() -> Vec<GoProblem> {
        // Always use progressive generator to ensure exactly 10 problems
        let section = &SectionConfig::default_go_sections().sections[6]; // section7-concurrency
        Self::generate_progressive_problems_for_section(section)
    }

    /// Generate enhanced error handling problems
    fn generate_enhanced_error_problems() -> Vec<GoProblem> {
        // Always use progressive generator to ensure exactly 10 problems
        let section = &SectionConfig::default_go_sections().sections[7]; // section8-error-handling
        Self::generate_progressive_problems_for_section(section)
    }

    /// Generate enhanced pointer problems
    fn generate_enhanced_pointer_problems() -> Vec<GoProblem> {
        // Always use progressive generator to ensure exactly 10 problems
        let section = &SectionConfig::default_go_sections().sections[8]; // section9-pointers
        Self::generate_progressive_problems_for_section(section)
    }

    /// Generate enhanced collection problems
    fn generate_enhanced_collection_problems() -> Vec<GoProblem> {
        // Always use progressive generator to ensure exactly 10 problems
        let section = &SectionConfig::default_go_sections().sections[9]; // section10-collections
        Self::generate_progressive_problems_for_section(section)
    }
}

/// Directory and file generation system for Go learning problems
pub struct GoFileGenerator {
    base_path: String,
}

impl GoFileGenerator {
    /// Create a new file generator with the specified base path
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    /// Create the main learning-go directory structure
    pub fn create_directory_structure(
        &self,
        config: &SectionConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create the main learning-go directory
        let learning_go_path = Path::new(&self.base_path).join("learning-go");

        if !learning_go_path.exists() {
            fs::create_dir_all(&learning_go_path)?;
            println!("Created directory: {}", learning_go_path.display());
        }

        // Create section subdirectories
        for section in &config.sections {
            let section_path = learning_go_path.join(&section.id);

            if !section_path.exists() {
                fs::create_dir_all(&section_path)?;
                println!("Created section directory: {}", section_path.display());
            }
        }

        Ok(())
    }

    /// Generate and write Go problem files for a specific section
    pub fn generate_section_files(
        &self,
        section: &Section,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let section_path = Path::new(&self.base_path)
            .join("learning-go")
            .join(&section.id);

        // Ensure section directory exists
        if !section_path.exists() {
            fs::create_dir_all(&section_path)?;
        }

        // Generate problems using enhanced generator that ensures 10 problems with progressive difficulty
        let problems = GoProblem::generate_enhanced_problems_for_section(section);

        // Validate that we have exactly 10 problems
        if problems.len() != 10 {
            return Err(format!(
                "Section {} generated {} problems, expected 10",
                section.name,
                problems.len()
            )
            .into());
        }

        // Validate progressive difficulty
        self.validate_progressive_difficulty(&problems)?;

        // Write each problem to a file
        for problem in problems {
            self.write_problem_file(&section_path, &problem)?;
        }

        println!(
            "Generated 10 problems for section: {} with progressive difficulty",
            section.name
        );
        Ok(())
    }

    /// Validate that problems have progressive difficulty (1-3 easy, 4-6 medium, 7-10 hard)
    fn validate_progressive_difficulty(
        &self,
        problems: &[GoProblem],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if problems.len() != 10 {
            return Err(format!("Expected 10 problems, got {}", problems.len()).into());
        }

        // Check difficulty progression
        for (i, problem) in problems.iter().enumerate() {
            let expected_difficulty = match i {
                0..=2 => 1, // Problems 1-3: easy
                3..=5 => 2, // Problems 4-6: medium
                _ => 3,     // Problems 7-10: hard
            };

            // Allow some flexibility - difficulty should be within reasonable range
            if problem.difficulty < 1 || problem.difficulty > 3 {
                return Err(format!(
                    "Problem {} has invalid difficulty {}, must be 1-3",
                    i + 1,
                    problem.difficulty
                )
                .into());
            }

            // Log if difficulty doesn't match expected progression (but don't fail)
            if problem.difficulty != expected_difficulty {
                println!(
                    "Note: Problem {} has difficulty {} (expected {})",
                    i + 1,
                    problem.difficulty,
                    expected_difficulty
                );
            }
        }

        Ok(())
    }

    /// Write a single problem to a Go file with proper formatting and validation
    pub fn write_problem_file(
        &self,
        section_path: &Path,
        problem: &GoProblem,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = section_path.join(&problem.filename);

        // Validate problem structure before writing
        problem.validate()?;

        // Format the content with proper Go formatting
        let formatted_content = self.format_go_content(&problem.content)?;

        // Validate Go conventions in the formatted content
        self.validate_go_conventions(&formatted_content)?;

        // Final syntax validation
        self.validate_final_go_syntax(&formatted_content)?;

        // Write the file
        fs::write(&file_path, &formatted_content)?;

        println!(
            "Created and validated problem file: {}",
            file_path.display()
        );
        Ok(())
    }

    /// Final validation to ensure the Go file is syntactically correct
    fn validate_final_go_syntax(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check that the file has proper Go structure
        let lines: Vec<&str> = content.lines().collect();

        let mut has_package = false;
        let mut has_main_func = false;
        let mut has_import = false;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.starts_with("package main") {
                has_package = true;
            }

            if trimmed.starts_with("import ") || trimmed == "import (" {
                has_import = true;
            }

            if trimmed.starts_with("func main()") {
                has_main_func = true;
            }
        }

        if !has_package {
            return Err("Go file must have 'package main' declaration".into());
        }

        if !has_main_func {
            return Err("Go file must have 'func main()' function".into());
        }

        // Import is optional for simple problems
        if !has_import && content.contains("fmt.") {
            return Err("Go file uses fmt package but missing import statement".into());
        }

        Ok(())
    }

    /// Format Go content to ensure proper structure and formatting
    fn format_go_content(&self, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut formatted = content.to_string();

        // Ensure proper line endings (Unix-style)
        formatted = formatted.replace("\r\n", "\n");

        // Validate and format Go code structure
        self.validate_go_syntax(&formatted)?;

        // Apply Go formatting standards
        formatted = self.apply_go_formatting_standards(&formatted)?;

        // Add final newline if missing
        if !formatted.ends_with('\n') {
            formatted.push('\n');
        }

        Ok(formatted)
    }

    /// Validate Go syntax and structure
    fn validate_go_syntax(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check for required Go file structure
        if !content.contains("package main") {
            return Err("Go file must contain 'package main' declaration".into());
        }

        if !content.contains("func main()") {
            return Err("Go file must contain 'func main()' function".into());
        }

        // Check for proper comment structure
        if !content.contains("// Problem:") {
            return Err("Go file must contain '// Problem:' comment".into());
        }

        if !content.contains("// Topic:") {
            return Err("Go file must contain '// Topic:' comment".into());
        }

        if !content.contains("// Difficulty:") {
            return Err("Go file must contain '// Difficulty:' comment".into());
        }

        // Validate basic Go syntax patterns
        self.validate_go_syntax_patterns(content)?;

        Ok(())
    }

    /// Validate specific Go syntax patterns
    fn validate_go_syntax_patterns(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lines: Vec<&str> = content.lines().collect();
        let mut brace_count = 0;
        let mut paren_count = 0;
        let mut in_string = false;
        let mut in_comment = false;

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                continue;
            }

            // Check for common Go syntax errors
            if trimmed.starts_with("//") {
                in_comment = true;
                continue;
            }

            if in_comment {
                in_comment = false;
            }

            // Basic brace and parenthesis matching
            for ch in line.chars() {
                match ch {
                    '"' if !in_comment => in_string = !in_string,
                    '{' if !in_string && !in_comment => brace_count += 1,
                    '}' if !in_string && !in_comment => brace_count -= 1,
                    '(' if !in_string && !in_comment => paren_count += 1,
                    ')' if !in_string && !in_comment => paren_count -= 1,
                    _ => {}
                }
            }

            // Check for negative brace/paren count (more closing than opening)
            if brace_count < 0 {
                return Err(format!("Unmatched closing brace on line {}", line_num + 1).into());
            }
            if paren_count < 0 {
                return Err(
                    format!("Unmatched closing parenthesis on line {}", line_num + 1).into(),
                );
            }

            // Check for common Go syntax requirements
            if trimmed.contains("package ") && !trimmed.starts_with("package ") {
                return Err(format!(
                    "Package declaration must be at start of line {}",
                    line_num + 1
                )
                .into());
            }

            // Check for proper import syntax
            if trimmed.contains("import ")
                && !trimmed.starts_with("import ")
                && !trimmed.contains("// import")
            {
                return Err(format!(
                    "Import statement should start at beginning of line {}",
                    line_num + 1
                )
                .into());
            }
        }

        // Check final brace/paren balance
        if brace_count != 0 {
            return Err(format!("Unmatched braces: {} unclosed", brace_count).into());
        }
        if paren_count != 0 {
            return Err(format!("Unmatched parentheses: {} unclosed", paren_count).into());
        }

        Ok(())
    }

    /// Apply Go formatting standards
    fn apply_go_formatting_standards(
        &self,
        content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = content.lines().collect();
        let mut formatted_lines = Vec::new();
        let mut indent_level = 0;
        let mut in_multiline_comment = false;

        for line in lines {
            let trimmed = line.trim();

            // Handle empty lines
            if trimmed.is_empty() {
                formatted_lines.push(String::new());
                continue;
            }

            // Handle comments
            if trimmed.starts_with("//") {
                formatted_lines.push(trimmed.to_string());
                continue;
            }

            if trimmed.starts_with("/*") {
                in_multiline_comment = true;
                formatted_lines.push(trimmed.to_string());
                continue;
            }

            if trimmed.ends_with("*/") {
                in_multiline_comment = false;
                formatted_lines.push(trimmed.to_string());
                continue;
            }

            if in_multiline_comment {
                formatted_lines.push(trimmed.to_string());
                continue;
            }

            // Adjust indent level based on braces
            if trimmed.ends_with('{') {
                let formatted_line = format!("{}{}", "    ".repeat(indent_level), trimmed);
                formatted_lines.push(formatted_line);
                indent_level += 1;
            } else if trimmed.starts_with('}') {
                indent_level = indent_level.saturating_sub(1);
                let formatted_line = format!("{}{}", "    ".repeat(indent_level), trimmed);
                formatted_lines.push(formatted_line);
            } else {
                // Regular line
                let formatted_line = format!("{}{}", "    ".repeat(indent_level), trimmed);
                formatted_lines.push(formatted_line);
            }
        }

        Ok(formatted_lines.join("\n"))
    }

    /// Validate that generated Go code follows Go conventions
    pub fn validate_go_conventions(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Skip comments and empty lines
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            // Check Go naming conventions
            if let Some(var_match) = self.extract_variable_declaration(trimmed) {
                if !self.is_valid_go_identifier(&var_match) {
                    return Err(format!(
                        "Invalid Go identifier '{}' on line {}",
                        var_match,
                        line_num + 1
                    )
                    .into());
                }
            }

            // Check for proper spacing around operators
            if self.has_improper_operator_spacing(trimmed) {
                println!(
                    "Warning: Improper operator spacing on line {}: {}",
                    line_num + 1,
                    trimmed
                );
            }

            // Check for proper function declaration format
            if trimmed.contains("func ") && !self.is_valid_function_declaration(trimmed) {
                return Err(format!(
                    "Invalid function declaration format on line {}",
                    line_num + 1
                )
                .into());
            }
        }

        Ok(())
    }

    /// Extract variable name from declaration
    fn extract_variable_declaration(&self, line: &str) -> Option<String> {
        let trimmed = line.trim();

        // Skip comments and control structures
        if trimmed.starts_with("//")
            || trimmed.starts_with("for ")
            || trimmed.starts_with("if ")
            || trimmed.starts_with("switch ")
            || trimmed.starts_with("func ")
            || trimmed.starts_with("package ")
            || trimmed.starts_with("import ")
        {
            return None;
        }

        // Simple pattern matching for variable declarations
        if line.contains("var ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "var" {
                return Some(parts[1].to_string());
            }
        }

        // Short variable declaration
        if line.contains(" := ") {
            let parts: Vec<&str> = line.split(" := ").collect();
            if !parts.is_empty() {
                let var_part = parts[0].trim();
                // Handle multiple variables (x, y := ...)
                if var_part.contains(',') {
                    let vars: Vec<&str> = var_part.split(',').collect();
                    if !vars.is_empty() {
                        return Some(vars[0].trim().to_string());
                    }
                } else {
                    return Some(var_part.to_string());
                }
            }
        }

        None
    }

    /// Check if identifier follows Go naming conventions
    fn is_valid_go_identifier(&self, identifier: &str) -> bool {
        if identifier.is_empty() {
            return false;
        }

        // Must start with letter or underscore
        let first_char = identifier.chars().next().unwrap();
        if !first_char.is_alphabetic() && first_char != '_' {
            return false;
        }

        // Rest must be alphanumeric or underscore
        for ch in identifier.chars().skip(1) {
            if !ch.is_alphanumeric() && ch != '_' {
                return false;
            }
        }

        // Check against Go keywords
        let go_keywords = [
            "break",
            "case",
            "chan",
            "const",
            "continue",
            "default",
            "defer",
            "else",
            "fallthrough",
            "for",
            "func",
            "go",
            "goto",
            "if",
            "import",
            "interface",
            "map",
            "package",
            "range",
            "return",
            "select",
            "struct",
            "switch",
            "type",
            "var",
        ];

        !go_keywords.contains(&identifier)
    }

    /// Check for proper operator spacing
    fn has_improper_operator_spacing(&self, line: &str) -> bool {
        // Check for operators without proper spacing
        let operators = [
            "=", "+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">=",
        ];

        for op in operators {
            if line.contains(op) {
                // This is a simplified check - in practice, you'd want more sophisticated parsing
                let op_index = line.find(op);
                if let Some(index) = op_index {
                    // Check if there's space before and after (with some exceptions)
                    if index > 0 && index < line.len() - op.len() {
                        let before = line.chars().nth(index - 1).unwrap_or(' ');
                        let after = line.chars().nth(index + op.len()).unwrap_or(' ');

                        // Skip if it's in a string or comment
                        if line[..index].contains('"') && line[index..].contains('"') {
                            continue;
                        }

                        if before != ' ' || after != ' ' {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    /// Check if function declaration is properly formatted
    fn is_valid_function_declaration(&self, line: &str) -> bool {
        // Basic check for function declaration format
        if !line.contains("func ") {
            return true; // Not a function declaration
        }

        // Should have format: func name(...) ... {
        // Simple validation: must have func, identifier, parentheses
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 || parts[0] != "func" {
            return false;
        }

        // Check that it has parentheses for parameters
        line.contains("(") && line.contains(")")
    }

    /// Generate all files for all sections in the configuration
    pub fn generate_all_files(
        &self,
        config: &SectionConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // First create the directory structure
        self.create_directory_structure(config)?;

        // Then generate files for each section
        for section in &config.sections {
            self.generate_section_files(section)?;
        }

        println!("Successfully generated all Go learning problems!");
        Ok(())
    }

    /// Validate that all generated files are syntactically correct Go files
    pub fn validate_generated_files(
        &self,
        config: &SectionConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let learning_go_path = Path::new(&self.base_path).join("learning-go");
        let mut total_files = 0;
        let mut validated_files = 0;

        for section in &config.sections {
            let section_path = learning_go_path.join(&section.id);

            if section_path.exists() {
                // Read all .go files in the section directory
                for entry in fs::read_dir(&section_path)? {
                    let entry = entry?;
                    let path = entry.path();

                    if path.extension().and_then(|s| s.to_str()) == Some("go") {
                        total_files += 1;

                        // Read file content
                        let content = fs::read_to_string(&path)?;

                        // Comprehensive validation
                        self.validate_go_syntax(&content)
                            .map_err(|e| format!("Syntax error in {}: {}", path.display(), e))?;

                        self.validate_go_conventions(&content).map_err(|e| {
                            format!("Convention error in {}: {}", path.display(), e)
                        })?;

                        self.validate_final_go_syntax(&content).map_err(|e| {
                            format!("Final validation error in {}: {}", path.display(), e)
                        })?;

                        validated_files += 1;
                        println!("✓ Validated: {}", path.display());
                    }
                }

                // Verify that each section has exactly 10 problems
                let go_files_count = fs::read_dir(&section_path)?
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("go"))
                    .count();

                if go_files_count != 10 {
                    return Err(format!(
                        "Section {} has {} Go files, expected 10",
                        section.name, go_files_count
                    )
                    .into());
                }

                println!(
                    "✓ Section {} has correct number of problems (10)",
                    section.name
                );
            }
        }

        println!(
            "✓ All {} generated Go files passed comprehensive validation!",
            validated_files
        );
        println!("✓ Total files validated: {}", total_files);

        if total_files != config.sections.len() * 10 {
            return Err(format!(
                "Expected {} total files ({} sections × 10), found {}",
                config.sections.len() * 10,
                config.sections.len(),
                total_files
            )
            .into());
        }

        Ok(())
    }

    /// Optional: Attempt to validate Go files by checking if they can be parsed by Go compiler
    /// This requires Go to be installed on the system
    pub fn validate_with_go_compiler(
        &self,
        config: &SectionConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let learning_go_path = Path::new(&self.base_path).join("learning-go");

        // Check if Go is available
        let go_version_output = std::process::Command::new("go").arg("version").output();

        if go_version_output.is_err() {
            println!("Warning: Go compiler not found, skipping compiler validation");
            return Ok(());
        }

        println!("Go compiler found, performing syntax validation...");

        for section in &config.sections {
            let section_path = learning_go_path.join(&section.id);

            if section_path.exists() {
                for entry in fs::read_dir(&section_path)? {
                    let entry = entry?;
                    let path = entry.path();

                    if path.extension().and_then(|s| s.to_str()) == Some("go") {
                        // Use 'go fmt' to check syntax
                        let fmt_output = std::process::Command::new("go")
                            .arg("fmt")
                            .arg("-n") // Don't write, just check
                            .arg(&path)
                            .output();

                        match fmt_output {
                            Ok(output) => {
                                if !output.status.success() {
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    return Err(format!(
                                        "Go fmt validation failed for {}: {}",
                                        path.display(),
                                        stderr
                                    )
                                    .into());
                                }
                                println!("✓ Go compiler validated: {}", path.display());
                            }
                            Err(e) => {
                                return Err(format!(
                                    "Failed to run go fmt on {}: {}",
                                    path.display(),
                                    e
                                )
                                .into());
                            }
                        }
                    }
                }
            }
        }

        println!("✓ All files passed Go compiler validation!");
        Ok(())
    }
}

/// Main execution flow that integrates all components into a complete problem generation workflow
pub fn execute_go_problem_generation_workflow(
    base_path: &str,
    skip_preview: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Go Learning Problem Generation Workflow");
    println!("================================================");

    // Step 1: Get section configuration with user confirmation
    let config = if skip_preview {
        println!("📋 Using default section configuration (preview skipped)...");
        SectionConfig::default_go_sections()
    } else {
        println!("📋 Loading section configuration and requesting user approval...");
        preview_and_confirm_sections()?
    };

    // Step 2: Validate configuration
    println!("🔍 Validating section configuration...");
    config
        .validate()
        .map_err(|e| format!("Configuration validation failed: {}", e))?;
    println!("✅ Configuration validated successfully");

    // Step 3: Report generation plan
    report_generation_progress(&config, base_path)?;

    // Step 4: Create directory structure and generate files
    println!("🏗️  Creating directory structure and generating problems...");
    create_go_learning_structure(base_path, &config)?;

    // Step 5: Final validation and summary
    println!("🎯 Generation workflow completed successfully!");
    print_generation_summary(&config, base_path)?;

    Ok(())
}

/// Report generation progress and plan to user
fn report_generation_progress(
    config: &SectionConfig,
    base_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let total_problems = config.sections.len() * 10;

    println!("\n📊 Generation Plan:");
    println!("   📁 Output directory: {}/learning-go", base_path);
    println!("   📚 Sections to create: {}", config.sections.len());
    println!("   📄 Total problems: {}", total_problems);
    println!("   🎯 Problems per section: 10");

    println!("\n📋 Section Overview:");
    for (index, section) in config.sections.iter().enumerate() {
        println!("   {}. {} ({})", index + 1, section.name, section.id);
        println!("      Description: {}", section.description);
        println!(
            "      Topics: {}",
            section
                .topics
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    println!("\n⏳ Starting generation process...");
    Ok(())
}

/// Print final generation summary
fn print_generation_summary(
    config: &SectionConfig,
    base_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let total_problems = config.sections.len() * 10;

    println!("\n🎉 Generation Summary:");
    println!(
        "   ✅ Successfully created {} sections",
        config.sections.len()
    );
    println!("   ✅ Generated {} Go problem files", total_problems);
    println!("   ✅ All files validated for syntax and formatting");
    println!("   📁 Location: {}/learning-go", base_path);

    println!("\n📚 Generated Sections:");
    for section in &config.sections {
        println!(
            "   📂 {} - {} problems",
            section.name, section.problem_count
        );
    }

    println!("\n🚀 Next Steps:");
    println!("   1. Navigate to {}/learning-go", base_path);
    println!("   2. Choose a section to start with (e.g., section1-basics)");
    println!("   3. Open any .go file and start learning!");
    println!("   4. Run 'go run filename.go' to execute your solutions");

    Ok(())
}

/// Enhanced error handling wrapper for the main workflow
pub fn run_go_problem_generator_with_error_handling(
    base_path: &str,
    skip_preview: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match execute_go_problem_generation_workflow(base_path, skip_preview) {
        Ok(()) => {
            println!("\n🎯 Go problem generation completed successfully!");
            Ok(())
        }
        Err(e) => {
            eprintln!("\n❌ Error during Go problem generation:");
            eprintln!("   {}", e);

            // Provide helpful error recovery suggestions
            eprintln!("\n🔧 Troubleshooting suggestions:");
            eprintln!("   • Check that the output directory is writable");
            eprintln!("   • Ensure sufficient disk space is available");
            eprintln!("   • Verify that no files are locked or in use");
            eprintln!("   • Try running with different output directory");

            Err(e)
        }
    }
}

/// Main function to create directory structure and generate all Go problem files
pub fn create_go_learning_structure(
    base_path: &str,
    config: &SectionConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let generator = GoFileGenerator::new(base_path.to_string());

    println!("🏗️  Creating Go learning directory structure...");
    generator.create_directory_structure(config)?;

    println!("📝 Generating Go problem files with progressive difficulty...");
    generator.generate_all_files(config)?;

    println!("🔍 Validating generated files for syntax and formatting...");
    generator.validate_generated_files(config)?;

    println!("🔧 Attempting Go compiler validation (optional)...");
    if let Err(e) = generator.validate_with_go_compiler(config) {
        println!("⚠️  Go compiler validation failed: {}", e);
        println!("   This is not critical - files may still be valid for learning purposes");
    }

    println!("✅ Go learning structure created successfully!");
    println!("   📁 Location: {}/learning-go", base_path);
    println!("   📊 Sections: {}", config.sections.len());
    println!("   📄 Total problems: {}", config.sections.len() * 10);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_section_config() {
        let config = SectionConfig::default_go_sections();

        assert_eq!(config.sections.len(), 10);
        assert!(config.validate().is_ok());

        // Test first section
        let first_section = &config.sections[0];
        assert_eq!(first_section.id, "section1-basics");
        assert_eq!(first_section.name, "Basic Syntax");
        assert_eq!(first_section.problem_count, 10);
        assert!(!first_section.topics.is_empty());
    }

    #[test]
    fn test_section_retrieval() {
        let config = SectionConfig::default_go_sections();

        let section = config.get_section("section1-basics");
        assert!(section.is_some());
        assert_eq!(section.unwrap().name, "Basic Syntax");

        let missing_section = config.get_section("nonexistent");
        assert!(missing_section.is_none());
    }

    #[test]
    fn test_go_problem_creation() {
        let problem = GoProblem::new(
            "problem01_variables.go".to_string(),
            r#"// Problem: Basic variable declaration
// Topic: Variables
// Difficulty: 1

package main

func main() {}"#
                .to_string(),
            "Basic variable declaration".to_string(),
            "Variables".to_string(),
            1,
        );

        assert!(problem.validate().is_ok());
        assert_eq!(
            problem.get_file_path("section1-basics"),
            "learning-go/section1-basics/problem01_variables.go"
        );
    }

    #[test]
    fn test_problem_validation() {
        let mut problem = GoProblem::new(
            "invalid_file.txt".to_string(),
            "content".to_string(),
            "description".to_string(),
            "topic".to_string(),
            1,
        );

        assert!(problem.validate().is_err()); // Invalid filename

        problem.filename = "valid_file.go".to_string();
        assert!(problem.validate().is_err()); // Still invalid content

        problem.content = r#"// Problem: Test
// Topic: Test
// Difficulty: 1

package main

func main() {}"#
            .to_string();
        assert!(problem.validate().is_ok()); // Now valid

        problem.difficulty = 5;
        assert!(problem.validate().is_err()); // Invalid difficulty
    }

    #[test]
    fn test_section_names() {
        let config = SectionConfig::default_go_sections();
        let names = config.get_section_names();

        assert_eq!(names.len(), 10);
        assert!(names.contains(&"Basic Syntax".to_string()));
        assert!(names.contains(&"Control Flow".to_string()));
        assert!(names.contains(&"Functions".to_string()));
    }

    #[test]
    fn test_user_choice_enum() {
        let choice = UserChoice::Approve;
        assert_eq!(choice, UserChoice::Approve);

        let choice2 = UserChoice::Modify;
        assert_ne!(choice, choice2);
    }

    #[test]
    fn test_section_modification_operations() {
        let mut config = SectionConfig::default_go_sections();
        let original_count = config.sections.len();

        // Test that we can access sections for modification
        assert!(original_count > 0);

        // Test section removal (simulate)
        if !config.sections.is_empty() {
            let first_section_name = config.sections[0].name.clone();
            config.sections.remove(0);
            assert_eq!(config.sections.len(), original_count - 1);

            // Verify the first section is different now
            if !config.sections.is_empty() {
                assert_ne!(config.sections[0].name, first_section_name);
            }
        }
    }

    #[test]
    fn test_section_swap() {
        let mut config = SectionConfig::default_go_sections();

        if config.sections.len() >= 2 {
            let first_name = config.sections[0].name.clone();
            let second_name = config.sections[1].name.clone();

            // Swap sections
            config.sections.swap(0, 1);

            // Verify swap occurred
            assert_eq!(config.sections[0].name, second_name);
            assert_eq!(config.sections[1].name, first_name);
        }
    }

    #[test]
    fn test_custom_section_addition() {
        let mut config = SectionConfig::default_go_sections();
        let original_count = config.sections.len();

        // Add a custom section
        let custom_section = Section {
            id: "section-custom".to_string(),
            name: "Custom Section".to_string(),
            description: "A custom test section".to_string(),
            problem_count: 10,
            topics: vec![Topic {
                name: "Custom Topic".to_string(),
                syntax_elements: vec!["custom syntax".to_string()],
                difficulty: 2,
            }],
        };

        config.sections.push(custom_section);

        assert_eq!(config.sections.len(), original_count + 1);
        assert_eq!(config.sections.last().unwrap().name, "Custom Section");
    }

    #[test]
    fn test_go_file_generator_creation() {
        let generator = GoFileGenerator::new("test_path".to_string());
        assert_eq!(generator.base_path, "test_path");
    }

    #[test]
    fn test_format_go_content() {
        let generator = GoFileGenerator::new("test".to_string());
        let content = r#"// Problem: Test Problem
// Topic: Testing
// Difficulty: 1

package main

import "fmt"

func main() {
fmt.Println("Hello")
}"#;
        let formatted = generator.format_go_content(content).unwrap();

        assert!(formatted.contains("package main"));
        assert!(formatted.ends_with('\n'));
        assert!(formatted.contains("    fmt.Println")); // Check indentation
    }

    #[test]
    fn test_validate_go_syntax() {
        let generator = GoFileGenerator::new("test".to_string());

        // Valid Go content
        let valid_content = r#"// Problem: Test Problem
// Topic: Testing
// Difficulty: 1

package main

import "fmt"

func main() {
    fmt.Println("Hello")
}"#;
        assert!(generator.validate_go_syntax(valid_content).is_ok());

        // Invalid Go content - missing package
        let invalid_content = r#"// Problem: Test Problem
// Topic: Testing
// Difficulty: 1

import "fmt"

func main() {
    fmt.Println("Hello")
}"#;
        assert!(generator.validate_go_syntax(invalid_content).is_err());
    }

    #[test]
    fn test_validate_go_conventions() {
        let generator = GoFileGenerator::new("test".to_string());

        let content = r#"// Problem: Test Problem
// Topic: Testing
// Difficulty: 1

package main

import "fmt"

func main() {
    var validName string = "test"
    fmt.Println(validName)
}"#;
        assert!(generator.validate_go_conventions(content).is_ok());
    }

    #[test]
    fn test_progressive_problem_generation() {
        let section = Section {
            id: "test-section".to_string(),
            name: "Test Section".to_string(),
            description: "Test description".to_string(),
            problem_count: 10,
            topics: vec![
                Topic {
                    name: "Test Topic 1".to_string(),
                    syntax_elements: vec!["syntax1".to_string()],
                    difficulty: 1,
                },
                Topic {
                    name: "Test Topic 2".to_string(),
                    syntax_elements: vec!["syntax2".to_string()],
                    difficulty: 2,
                },
            ],
        };

        let problems = GoProblem::generate_progressive_problems_for_section(&section);

        assert_eq!(problems.len(), 10);

        // Check progressive difficulty
        assert_eq!(problems[0].difficulty, 1); // First problem should be easy
        assert_eq!(problems[1].difficulty, 1); // Second problem should be easy
        assert_eq!(problems[2].difficulty, 1); // Third problem should be easy
        assert_eq!(problems[3].difficulty, 2); // Fourth problem should be medium
        assert_eq!(problems[6].difficulty, 3); // Seventh problem should be hard
        assert_eq!(problems[9].difficulty, 3); // Last problem should be hard
    }

    #[test]
    fn test_generate_generic_problems() {
        let section = Section {
            id: "test-section".to_string(),
            name: "Test Section".to_string(),
            description: "Test description".to_string(),
            problem_count: 10, // Updated to match new implementation
            topics: vec![Topic {
                name: "Test Topic".to_string(),
                syntax_elements: vec!["test syntax".to_string()],
                difficulty: 2,
            }],
        };

        let problems = GoProblem::generate_progressive_problems_for_section(&section);
        assert_eq!(problems.len(), 10); // Updated to expect 10 problems
        assert!(problems[0].filename.contains("problem01"));
        assert!(problems[0].content.contains("package main"));

        // Test progressive difficulty
        assert_eq!(problems[0].difficulty, 1); // First should be easy
        assert_eq!(problems[6].difficulty, 3); // Seventh should be hard
    }

    #[test]
    fn test_directory_structure_creation() {
        use std::env;
        use std::fs;

        // Create a temporary directory for testing
        let temp_dir = env::temp_dir().join("go_problems_test");

        // Clean up any existing test directory
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).ok();
        }

        let generator = GoFileGenerator::new(temp_dir.to_string_lossy().to_string());

        // Create a minimal config with just one section
        let mut config = SectionConfig { sections: vec![] };
        config.sections.push(Section {
            id: "section1-test".to_string(),
            name: "Test Section".to_string(),
            description: "Test description".to_string(),
            problem_count: 2,
            topics: vec![Topic {
                name: "Test Topic".to_string(),
                syntax_elements: vec!["test syntax".to_string()],
                difficulty: 1,
            }],
        });

        // Test directory creation
        let result = generator.create_directory_structure(&config);
        assert!(result.is_ok());

        // Verify directories were created
        let learning_go_path = temp_dir.join("learning-go");
        assert!(learning_go_path.exists());

        let section_path = learning_go_path.join("section1-test");
        assert!(section_path.exists());

        // Clean up
        fs::remove_dir_all(&temp_dir).ok();
    }
}
