# Contributing Guide

Thank you for considering contributing to OutOf5K! This document outlines how to contribute effectively.

## Getting Started

1. **Read the documentation**
   - [Developer Setup](./developer-setup.md)
   - [Project Structure](./project-structure.md)
   - [Architecture Overview](../architecture/overview.md)

2. **Set up your environment**
   - Follow the [Developer Setup Guide](./developer-setup.md)

3. **Find something to work on**
   - Check [GitHub Issues](https://github.com/your-org/outof5k/issues)
   - Look for `good first issue` labels
   - Or propose a new feature

## Contribution Process

### 1. Discuss First

For significant changes, open an issue to discuss before implementing:

- **Bug reports**: Describe the bug, steps to reproduce, expected behavior
- **Feature requests**: Describe the use case, proposed solution, alternatives considered
- **Questions**: Use GitHub Discussions for questions

### 2. Fork and Branch

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/outof5k.git
cd outof5k
git remote add upstream https://github.com/your-org/outof5k.git

# Create a branch
git checkout -b feature/my-contribution
```

### 3. Make Changes

- Follow our [code style guidelines](#code-style)
- Write tests for new functionality
- Update documentation as needed
- Keep commits focused and atomic

### 4. Test Your Changes

```bash
# Run all tests
pnpm test
cd src-tauri && cargo test
cd python && pytest

# Run linters
pnpm lint
cd src-tauri && cargo clippy
cd python && ruff check .
```

### 5. Submit a Pull Request

1. Push your branch to your fork
2. Open a PR against `develop` branch
3. Fill out the PR template
4. Wait for review

### 6. Address Feedback

- Respond to review comments
- Make requested changes
- Push updates to your branch (PR updates automatically)

## Code Style

### TypeScript/Svelte

- Use TypeScript for all new code
- Follow ESLint and Prettier configurations
- Use meaningful variable and function names
- Add JSDoc comments for public APIs

```typescript
// Good
async function importDemoFiles(paths: string[]): Promise<ImportResult> {
  // Implementation
}

// Avoid
async function doThing(p: any) {
  // Implementation
}
```

### Rust

- Follow `rustfmt` formatting
- Use `clippy` suggestions
- Document public items with `///` comments
- Use meaningful error types

```rust
// Good
/// Imports demo files from the given paths.
/// 
/// # Errors
/// Returns an error if any file cannot be read or parsed.
pub async fn import_demos(&self, paths: &[String]) -> Result<Vec<ImportResult>> {
    // Implementation
}

// Avoid
pub async fn import(p: Vec<String>) -> Result<Vec<ImportResult>> {
    // Implementation
}
```

### Python

- Follow PEP 8 style guide
- Use Black for formatting
- Use Ruff for linting
- Add type hints to all functions

```python
# Good
def parse_demo(file_path: Path, options: ParseOptions) -> ParsedDemo:
    """Parse a CS2 demo file.
    
    Args:
        file_path: Path to the demo file
        options: Parsing configuration
        
    Returns:
        Parsed demo data
        
    Raises:
        ParseError: If the file cannot be parsed
    """
    pass

# Avoid
def parse(f, opts):
    pass
```

## Commit Guidelines

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation changes |
| `style` | Code style changes (formatting, etc.) |
| `refactor` | Code refactoring |
| `test` | Adding or updating tests |
| `chore` | Maintenance tasks |
| `perf` | Performance improvements |

### Examples

```
feat(parser): add support for CS2 demo format v4

Implements parsing for the new demo format introduced in the
latest CS2 update. The new format includes additional metadata
fields and a modified event structure.

Closes #123

---

fix(ui): handle empty demo list gracefully

Previously the app would crash when the demo list was empty.
Now it shows an empty state message instead.

Fixes #456
```

## Pull Request Guidelines

### PR Title

Use the same format as commit messages:

```
feat(component): add feature description
```

### PR Description

Include:

1. **Summary**: What does this PR do?
2. **Motivation**: Why is this change needed?
3. **Changes**: List of specific changes
4. **Testing**: How was this tested?
5. **Screenshots**: If UI changes (before/after)

### PR Checklist

- [ ] Tests pass locally
- [ ] Linting passes
- [ ] Documentation updated (if needed)
- [ ] Commits are clean and well-described
- [ ] PR description is complete

## Review Process

### What We Look For

1. **Correctness**: Does the code do what it's supposed to?
2. **Tests**: Are there adequate tests?
3. **Style**: Does it follow our guidelines?
4. **Performance**: Are there any performance concerns?
5. **Security**: Are there any security issues?
6. **Documentation**: Is it documented appropriately?

### Review Timeline

- Simple changes: 1-2 days
- Medium changes: 3-5 days
- Large changes: 1-2 weeks

### Addressing Feedback

When reviewers request changes:

1. Address all comments
2. Reply to each comment explaining what you did
3. Request re-review when ready

## Types of Contributions

### Code Contributions

- Bug fixes
- New features
- Performance improvements
- Refactoring

### Documentation

- Fix typos or errors
- Add missing documentation
- Improve existing docs
- Add examples

### Testing

- Add missing tests
- Improve test coverage
- Fix flaky tests

### Design

- UI/UX improvements
- Visual design contributions
- Accessibility improvements

### Community

- Answer questions in Discussions
- Help triage issues
- Review PRs

## Code of Conduct

### Our Standards

- Be respectful and inclusive
- Accept constructive criticism gracefully
- Focus on what's best for the community
- Show empathy towards others

### Unacceptable Behavior

- Harassment or discrimination
- Trolling or insulting comments
- Personal or political attacks
- Publishing others' private information

### Enforcement

Violations may result in temporary or permanent bans from the project.

## Recognition

Contributors are recognized in:

- The project README
- Release notes
- The contributors page

## Questions?

- Open a [GitHub Discussion](https://github.com/your-org/outof5k/discussions)
- Join our [Discord server](https://discord.gg/outof5k) (if applicable)
- Email: maintainers@outof5k.app (if applicable)

---

Thank you for contributing to OutOf5K! Your help makes this project better for everyone.
