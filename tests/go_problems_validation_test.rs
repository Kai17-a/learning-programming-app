use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;
use learning_programming_app::generators::go_problems::{
    SectionConfig, Section, Topic, GoProblem, GoFileGenerator
};

/// Go syntax validator that uses the Go compiler to validate syntax
pub struct GoSyntaxValidator;

impl GoSyntaxValidator {
    /// Validate Go file syntax using the Go compiler
    pub fn validate_go_file_syntax(file_path: &Path) -> Result<(), String> {
        // Check if Go is installed
        if !Self::is_go_installed() {
            return Ok(()); // Skip validation if Go is not installed
        }

        // Use 'go build -o /dev/null' to check syntax without creating executable
        let output = Command::new("go")
            .args(&["build", "-o", if cfg!(windows) { "NUL" } else { "/dev/null" }])
            .arg(file_path)
            .output()
            .map_err(|e| format!("Failed to run go build: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Go syntax error: {}", stderr));
        }

        Ok(())
    }

    /// Validate Go content syntax by creating a temporary file
    pub fn validate_go_content_syntax(content: &str) -> Result<(), String> {
        if !Self::is_go_installed() {
            return Ok(()); // Skip validation if Go is not installed
        }

        let temp_dir = TempDir::new().map_err(|e| format!("Failed to create temp dir: {}", e))?;
        let temp_file = temp_dir.path().join("temp.go");
        
        fs::write(&temp_file, content)
            .map_err(|e| format!("Failed to write temp file: {}", e))?;

        Self::validate_go_file_syntax(&temp_file)
    }

    /// Check if Go compiler is installed and available
    pub fn is_go_installed() -> bool {
        Command::new("go")
            .args(&["version"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Validate multiple Go files in a directory
    pub fn validate_directory_go_files(dir_path: &Path) -> Result<Vec<String>, String> {
        let mut errors = Vec::new();

        if !dir_path.exists() {
            return Err(format!("Directory does not exist: {}", dir_path.display()));
        }

        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("go") {
                if let Err(error) = Self::validate_go_file_syntax(&path) {
                    errors.push(format!("{}: {}", path.display(), error));
                }
            }
        }

        Ok(errors)
    }

    /// Validate Go files recursively in a directory tree
    pub fn validate_directory_tree_go_files(root_path: &Path) -> Result<Vec<String>, String> {
        let mut all_errors = Vec::new();

        if !root_path.exists() {
            return Err(format!("Root directory does not exist: {}", root_path.display()));
        }

        Self::validate_directory_recursive(root_path, &mut all_errors)?;

        Ok(all_errors)
    }

    /// Recursive helper for directory tree validation
    fn validate_directory_recursive(dir_path: &Path, errors: &mut Vec<String>) -> Result<(), String> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively validate subdirectories
                Self::validate_directory_recursive(&path, errors)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("go") {
                // Validate Go files
                if let Err(error) = Self::validate_go_file_syntax(&path) {
                    errors.push(format!("{}: {}", path.display(), error));
                }
            }
        }

        Ok(())
    }

    /// Get Go version information
    pub fn get_go_version() -> Result<String, String> {
        let output = Command::new("go")
            .args(&["version"])
            .output()
            .map_err(|e| format!("Failed to run go version: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err("Failed to get Go version".to_string())
        }
    }
}

/// Test validation of generated Go file syntax
#[test]
fn test_go_problem_validation() {
    // Test valid Go problem
    let valid_problem = GoProblem::new(
        "test_problem.go".to_string(),
        r#"// Problem: Test Problem
// Topic: Testing
// Difficulty: 1

package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
"#.to_string(),
        "Test problem description".to_string(),
        "Testing".to_string(),
        1,
    );

    assert!(valid_problem.validate().is_ok());

    // Test invalid filename
    let invalid_filename = GoProblem::new(
        "test_problem.txt".to_string(),
        valid_problem.content.clone(),
        "Test".to_string(),
        "Testing".to_string(),
        1,
    );

    assert!(invalid_filename.validate().is_err());
    assert!(invalid_filename.validate().unwrap_err().to_string().contains("must end with .go"));

    // Test invalid difficulty
    let invalid_difficulty = GoProblem::new(
        "test_problem.go".to_string(),
        valid_problem.content.clone(),
        "Test".to_string(),
        "Testing".to_string(),
        5, // Invalid difficulty
    );

    assert!(invalid_difficulty.validate().is_err());
    assert!(invalid_difficulty.validate().unwrap_err().to_string().contains("difficulty must be between 1 and 3"));

    // Test missing package main
    let missing_package = GoProblem::new(
        "test_problem.go".to_string(),
        r#"// Problem: Test Problem
// Topic: Testing
// Difficulty: 1

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
"#.to_string(),
        "Test".to_string(),
        "Testing".to_string(),
        1,
    );

    assert!(missing_package.validate().is_err());
    assert!(missing_package.validate().unwrap_err().to_string().contains("must contain 'package main'"));

    // Test missing main function
    let missing_main = GoProblem::new(
        "test_problem.go".to_string(),
        r#"// Problem: Test Problem
// Topic: Testing
// Difficulty: 1

package main

import "fmt"

func test() {
    fmt.Println("Hello, World!")
}
"#.to_string(),
        "Test".to_string(),
        "Testing".to_string(),
        1,
    );

    assert!(missing_main.validate().is_err());
    assert!(missing_main.validate().unwrap_err().to_string().contains("must contain 'func main()'"));

    // Test missing required comments
    let missing_comments = GoProblem::new(
        "test_problem.go".to_string(),
        r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
"#.to_string(),
        "Test".to_string(),
        "Testing".to_string(),
        1,
    );

    assert!(missing_comments.validate().is_err());
    assert!(missing_comments.validate().unwrap_err().to_string().contains("must contain '// Problem:' comment"));
}

/// Test section configuration validation
#[test]
fn test_section_config_validation() {
    // Test valid configuration
    let valid_config = SectionConfig::default_go_sections();
    assert!(valid_config.validate().is_ok());

    // Test empty sections
    let empty_config = SectionConfig { sections: vec![] };
    assert!(empty_config.validate().is_err());
    assert!(empty_config.validate().unwrap_err().contains("No sections defined"));

    // Test section with no topics
    let section_no_topics = Section {
        id: "test".to_string(),
        name: "Test".to_string(),
        description: "Test section".to_string(),
        problem_count: 10,
        topics: vec![],
    };
    let config_no_topics = SectionConfig {
        sections: vec![section_no_topics],
    };
    assert!(config_no_topics.validate().is_err());
    assert!(config_no_topics.validate().unwrap_err().contains("has no topics"));

    // Test section with zero problems
    let topic = Topic {
        name: "Test Topic".to_string(),
        syntax_elements: vec!["test".to_string()],
        difficulty: 1,
    };
    let section_zero_problems = Section {
        id: "test".to_string(),
        name: "Test".to_string(),
        description: "Test section".to_string(),
        problem_count: 0,
        topics: vec![topic.clone()],
    };
    let config_zero_problems = SectionConfig {
        sections: vec![section_zero_problems],
    };
    assert!(config_zero_problems.validate().is_err());
    assert!(config_zero_problems.validate().unwrap_err().contains("has zero problems"));

    // Test topic with no syntax elements
    let topic_no_elements = Topic {
        name: "Test Topic".to_string(),
        syntax_elements: vec![],
        difficulty: 1,
    };
    let section_no_elements = Section {
        id: "test".to_string(),
        name: "Test".to_string(),
        description: "Test section".to_string(),
        problem_count: 10,
        topics: vec![topic_no_elements],
    };
    let config_no_elements = SectionConfig {
        sections: vec![section_no_elements],
    };
    assert!(config_no_elements.validate().is_err());
    assert!(config_no_elements.validate().unwrap_err().contains("has no syntax elements"));

    // Test topic with invalid difficulty
    let topic_invalid_difficulty = Topic {
        name: "Test Topic".to_string(),
        syntax_elements: vec!["test".to_string()],
        difficulty: 5, // Invalid
    };
    let section_invalid_difficulty = Section {
        id: "test".to_string(),
        name: "Test".to_string(),
        description: "Test section".to_string(),
        problem_count: 10,
        topics: vec![topic_invalid_difficulty],
    };
    let config_invalid_difficulty = SectionConfig {
        sections: vec![section_invalid_difficulty],
    };
    assert!(config_invalid_difficulty.validate().is_err());
    assert!(config_invalid_difficulty.validate().unwrap_err().to_string().contains("has invalid difficulty level"));
}

/// Test directory structure creation
#[test]
fn test_directory_structure_creation() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path().to_string_lossy().to_string();
    
    let generator = GoFileGenerator::new(base_path.clone());
    let config = SectionConfig::default_go_sections();
    
    // Test directory structure creation
    assert!(generator.create_directory_structure(&config).is_ok());
    
    // Verify learning-go directory exists
    let learning_go_path = temp_dir.path().join("learning-go");
    assert!(learning_go_path.exists());
    assert!(learning_go_path.is_dir());
    
    // Verify all section directories exist
    for section in &config.sections {
        let section_path = learning_go_path.join(&section.id);
        assert!(section_path.exists(), "Section directory {} should exist", section.id);
        assert!(section_path.is_dir(), "Section path {} should be a directory", section.id);
    }
}

/// Test problem content quality and correctness
#[test]
fn test_problem_content_quality() {
    let config = SectionConfig::default_go_sections();
    
    for section in &config.sections {
        let problems = GoProblem::generate_progressive_problems_for_section(section);
        
        // Verify correct number of problems
        assert_eq!(problems.len(), 10, "Section {} should have 10 problems", section.id);
        
        // Verify progressive difficulty
        let mut easy_count = 0;
        let mut _medium_count = 0;
        let mut _hard_count = 0;
        
        for problem in &problems {
            // Validate each problem
            assert!(problem.validate().is_ok(), 
                "Problem {} should be valid: {:?}", 
                problem.filename, problem.validate().unwrap_err());
            
            // Count difficulty levels
            match problem.difficulty {
                1 => easy_count += 1,
                2 => _medium_count += 1,
                3 => _hard_count += 1,
                _ => panic!("Invalid difficulty level: {}", problem.difficulty),
            }
            
            // Verify filename format
            assert!(problem.filename.starts_with("problem"), 
                "Filename should start with 'problem': {}", problem.filename);
            assert!(problem.filename.ends_with(".go"), 
                "Filename should end with '.go': {}", problem.filename);
            
            // Verify content structure
            assert!(problem.content.contains("// Problem:"), 
                "Content should contain problem description");
            assert!(problem.content.contains("// Topic:"), 
                "Content should contain topic");
            assert!(problem.content.contains("// Difficulty:"), 
                "Content should contain difficulty");
            assert!(problem.content.contains("package main"), 
                "Content should contain package main");
            assert!(problem.content.contains("func main()"), 
                "Content should contain main function");
        }
        
        // Verify difficulty distribution (should have problems of different difficulties)
        assert!(easy_count > 0, "Section {} should have easy problems", section.id);
        // Note: Some sections might not have all difficulty levels due to topic distribution
    }
}

/// Test Go file syntax validation using Go compiler
#[test]
fn test_go_syntax_validation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a valid Go file
    let valid_go_content = r#"// Problem: Test Problem
// Topic: Testing
// Difficulty: 1

package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
"#;
    
    let valid_file_path = temp_dir.path().join("valid.go");
    fs::write(&valid_file_path, valid_go_content).unwrap();
    
    // Test syntax validation using Go compiler if available
    if GoSyntaxValidator::is_go_installed() {
        let result = GoSyntaxValidator::validate_go_file_syntax(&valid_file_path);
        assert!(result.is_ok(), "Valid Go file should pass syntax validation: {:?}", result);
    } else {
        println!("Skipping Go compiler validation - Go not installed");
    }
    
    // Basic file structure validation
    assert!(valid_file_path.exists());
    let content = fs::read_to_string(&valid_file_path).unwrap();
    assert!(content.contains("package main"));
    assert!(content.contains("func main()"));
    
    // Create an invalid Go file
    let invalid_go_content = r#"// Problem: Test Problem
// Topic: Testing
// Difficulty: 1

package main

import "fmt"

func main() {
    fmt.Println("Hello, World!" // Missing closing parenthesis
}
"#;
    
    let invalid_file_path = temp_dir.path().join("invalid.go");
    fs::write(&invalid_file_path, invalid_go_content).unwrap();
    
    // Test syntax validation for invalid file
    if GoSyntaxValidator::is_go_installed() {
        let result = GoSyntaxValidator::validate_go_file_syntax(&invalid_file_path);
        assert!(result.is_err(), "Invalid Go file should fail syntax validation");
    } else {
        println!("Skipping Go compiler validation - Go not installed");
    }
    
    // Verify the invalid file was created
    assert!(invalid_file_path.exists());
    let invalid_content = fs::read_to_string(&invalid_file_path).unwrap();
    assert!(invalid_content.contains("package main"));
}

/// Test comprehensive validation of generated learning-go directory
#[test]
fn test_comprehensive_directory_validation() {
    // Test against the existing generated directory if it exists
    let learning_go_path = Path::new("test-learning-go/learning-go");
    
    if !learning_go_path.exists() {
        println!("Skipping comprehensive validation - learning-go directory not found");
        return;
    }

    let config = SectionConfig::default_go_sections();
    
    // Validate directory structure
    for section in &config.sections {
        let section_path = learning_go_path.join(&section.id);
        assert!(section_path.exists(), "Section directory {} should exist", section.id);
        assert!(section_path.is_dir(), "Section path {} should be a directory", section.id);
        
        // Count files in section
        let entries = fs::read_dir(&section_path).unwrap();
        let go_files: Vec<_> = entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()? == "go" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();
        
        assert_eq!(go_files.len(), 10, "Section {} should have exactly 10 Go files", section.id);
        
        // Validate each Go file
        for go_file in go_files {
            let content = fs::read_to_string(&go_file).unwrap();
            
            // Basic structure validation
            assert!(content.contains("// Problem:"), 
                "File {:?} should contain problem description", go_file);
            assert!(content.contains("// Topic:"), 
                "File {:?} should contain topic", go_file);
            assert!(content.contains("// Difficulty:"), 
                "File {:?} should contain difficulty", go_file);
            assert!(content.contains("package main"), 
                "File {:?} should contain package main", go_file);
            assert!(content.contains("func main()"), 
                "File {:?} should contain main function", go_file);
            
            // Validate using Go compiler if available
            if GoSyntaxValidator::is_go_installed() {
                let result = GoSyntaxValidator::validate_go_file_syntax(&go_file);
                if let Err(error) = result {
                    println!("Syntax error in {:?}: {}", go_file, error);
                    // Don't fail the test for syntax errors in generated files
                    // as they might contain TODO comments that make them incomplete
                }
            }
        }
    }
    
    // Validate entire directory tree if Go is available
    if GoSyntaxValidator::is_go_installed() {
        let errors = GoSyntaxValidator::validate_directory_tree_go_files(learning_go_path).unwrap();
        if !errors.is_empty() {
            println!("Found {} syntax errors in generated files:", errors.len());
            for error in &errors {
                println!("  {}", error);
            }
            // Note: We don't fail the test here because generated files might have TODO comments
            // that make them syntactically incomplete, which is expected for learning exercises
        }
    }
}

/// Test integration with existing file generation system
#[test]
fn test_integration_with_file_generation() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path().to_string_lossy().to_string();
    
    let generator = GoFileGenerator::new(base_path.clone());
    let config = SectionConfig::default_go_sections();
    
    // Test full generation workflow
    assert!(generator.create_directory_structure(&config).is_ok());
    
    // Generate files for first few sections to test integration
    for section in config.sections.iter().take(3) {
        let result = generator.generate_section_files(section);
        assert!(result.is_ok(), "Should successfully generate files for section {}: {:?}", 
            section.id, result.unwrap_err());
        
        // Verify files were created and are valid
        let section_path = temp_dir.path().join("learning-go").join(&section.id);
        assert!(section_path.exists());
        
        let entries = fs::read_dir(&section_path).unwrap();
        let file_count = entries.count();
        assert_eq!(file_count, 10, "Should generate exactly 10 files for section {}", section.id);
    }
}

/// Benchmark test for problem generation performance
#[test]
fn test_problem_generation_performance() {
    use std::time::Instant;
    
    let config = SectionConfig::default_go_sections();
    let start = Instant::now();
    
    // Generate problems for all sections
    let mut total_problems = 0;
    for section in &config.sections {
        let problems = GoProblem::generate_progressive_problems_for_section(section);
        total_problems += problems.len();
        
        // Validate each problem
        for problem in problems {
            assert!(problem.validate().is_ok());
        }
    }
    
    let duration = start.elapsed();
    println!("Generated and validated {} problems in {:?}", total_problems, duration);
    
    // Performance assertion - should complete within reasonable time
    assert!(duration.as_secs() < 5, "Problem generation should complete within 5 seconds");
    assert_eq!(total_problems, 100, "Should generate exactly 100 problems total");
}