library(fs)

# - [x] Ch 01 - Getting Started
# - [x] Ch 02 - Programming a Guessing Game
# - [x] Ch 03 -  Common Programming Concepts
# - [ ] Ch 04 - Understanding Ownership
# - [ ] Ch 05 - Using Structs to Structure Related Data
# - [ ] Ch 06 - Enums and Pattern Matching
# - [ ] Ch 08 - Common Collections
# - [ ] Ch 10 - Generic Types, Traits, and Lifetimes
# - [ ] Ch 13 - Function Language Features: Iterators and Closures
# - [ ] Ch 18 - Patterns and Matching

fs::file_copy("template.qmd", "01-getting-started.qmd")
fs::file_copy("template.qmd", "02-guessing-game.qmd")
fs::file_copy("template.qmd", "03-concepts.qmd")
fs::file_copy("template.qmd", "04-ownership.qmd")

