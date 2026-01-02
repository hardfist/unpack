# Agents

This document outlines the different types of agents available for handling various tasks in the unpack project.

## Available Agent Types

### General Agent
- **Purpose**: General-purpose agent for researching complex questions and executing multi-step tasks
- **Capabilities**: Can execute multiple units of work in parallel
- **Use Cases**: 
  - Complex multi-step implementations
  - Research tasks requiring multiple operations
  - Coordinating different aspects of development

### Explore Agent
- **Purpose**: Fast agent specialized for exploring codebases
- **Capabilities**: 
  - Quick file pattern searches
  - Code keyword searches
  - Codebase structure analysis
- **Thoroughness Levels**:
  - `quick`: Basic searches and quick overviews
  - `medium`: Moderate exploration with some depth
  - `very thorough`: Comprehensive analysis across multiple locations and naming conventions
- **Use Cases**:
  - Finding files by patterns (e.g., `src/components/**/*.tsx`)
  - Searching for specific code keywords or patterns
  - Understanding how systems work (e.g., "how do API endpoints work?")
  - Codebase exploration and documentation

## Usage Guidelines

### When to Use Task Tool with Agents
- Complex multistep tasks requiring coordination
- Codebase exploration beyond simple file/function lookups
- Research questions about code architecture
- Custom slash command execution

### When to Use Direct Tools Instead
- Reading specific known file paths → Use Read or Glob
- Searching for specific class definitions → Use Glob
- Searching within 2-3 specific files → Use Read
- Simple, single-step operations

## Best Practices

1. **Parallel Execution**: Launch multiple agents concurrently when possible for maximum performance
2. **Detailed Prompts**: Provide highly detailed task descriptions for autonomous execution
3. **Clear Expectations**: Specify whether agents should write code or just do research
4. **Stateless Design**: Each agent invocation is independent unless a session_id is provided
5. **Trust Results**: Agent outputs should generally be trusted and used as authoritative

## Examples

```markdown
# Explore codebase structure
Task(subagent_type="explore", thoroughness="medium", description="Analyze structure", prompt="Analyze the overall codebase structure and identify main components")

# Research implementation patterns
Task(subagent_type="general", description="Research patterns", prompt="Research how error handling is implemented across the codebase and suggest improvements")

# Execute custom command
Task(subagent_type="general", description="Check file", prompt="/check-file path/to/file.py")
```

This agent system enables efficient and organized handling of complex development tasks while maintaining clear separation of concerns.