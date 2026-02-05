# ADR-005: PlantUML for Technical Diagrams

## Status

Accepted

## Date

2024-01-15

## Context

OutOf5K requires comprehensive technical documentation including:
- C4 architecture diagrams (Context, Container, Component, Code)
- UML diagrams (Sequence, Class, State)
- Data flow diagrams

Documentation needs to be:
- Version controlled alongside code
- Easy to update as the system evolves
- Exportable to PDF via LaTeX
- Viewable without special tools (ideally)
- Maintainable by developers (not requiring design tools)

Options considered:
- **PlantUML**: Text-based diagrams, mature, wide tool support
- **Mermaid**: Text-based, GitHub native rendering
- **Structurizr DSL**: Purpose-built for C4 diagrams
- **Draw.io/Excalidraw**: GUI-based diagram tools
- **TikZ**: Native LaTeX diagrams

## Decision

We will use **PlantUML** for all technical diagrams.

## Consequences

### Positive

1. **Text-Based**: Diagrams are defined in plain text, enabling:
   - Git version control with meaningful diffs
   - Code review of diagram changes
   - Automated generation in CI/CD

2. **Excellent C4 Support**: The C4-PlantUML library provides first-class C4 diagram support:
   ```plantuml
   !include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml
   Container(app, "Application", "Tauri", "Desktop app")
   ```

3. **Comprehensive UML Support**: Native support for all standard UML diagrams (sequence, class, state, activity, etc.).

4. **LaTeX Integration**: PlantUML outputs to PDF, PNG, SVG - all easily included in LaTeX documents.

5. **Wide Tool Support**:
   - VS Code extension (live preview)
   - IntelliJ plugin
   - Online editor (plantuml.com)
   - CLI tool for batch processing

6. **Mature and Stable**: PlantUML has been around since 2009, with extensive documentation and community.

### Negative

1. **Java Dependency**: PlantUML requires Java to run. Minor inconvenience for developers.

2. **Learning Curve**: PlantUML syntax takes time to learn, especially for complex diagrams.

3. **Limited Layout Control**: Automatic layout sometimes produces suboptimal results. Workarounds exist but can be tedious.

4. **No Inline GitHub Rendering**: Unlike Mermaid, PlantUML doesn't render natively in GitHub Markdown. Requires image generation.

### Neutral

1. **Not Real-Time Collaborative**: Unlike GUI tools, multiple people can't edit simultaneously. Trade-off for version control benefits.

## Diagram Standards

### File Organization

```
docs/architecture/
├── c4/
│   ├── 1-context.puml
│   ├── 2-container.puml
│   └── 3-component-*.puml
└── uml/
    ├── sequence/
    ├── class/
    └── state/
```

### Naming Conventions

- C4 diagrams: `{level}-{name}.puml` (e.g., `2-container.puml`)
- Sequence diagrams: `{feature}-sequence.puml` (e.g., `demo-import-sequence.puml`)
- Class diagrams: `{domain}-class.puml` (e.g., `domain-models-class.puml`)

### Standard Includes

All diagrams should include appropriate theme and stdlib imports:

```plantuml
@startuml
!theme plain
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml

title Diagram Title

' ... diagram content ...

@enduml
```

## Workflow

### Editing

1. Edit `.puml` files in VS Code with PlantUML extension
2. Live preview shows diagram as you type
3. Commit changes to git

### Building

```bash
# Generate all diagrams as PNG
make diagrams

# Or manually:
java -jar plantuml.jar docs/**/*.puml
```

### CI/CD

GitHub Actions workflow generates images on push:

```yaml
- name: Generate Diagrams
  run: |
    java -jar plantuml.jar -tpng docs/**/*.puml
    java -jar plantuml.jar -tsvg docs/**/*.puml
```

## Alternatives Considered

### Alternative 1: Mermaid

Native GitHub rendering, simpler syntax.

**Why Rejected:**
- C4 support is experimental and limited
- Less expressive for complex diagrams
- Fewer customization options
- PlantUML's maturity and features outweigh Mermaid's convenience

### Alternative 2: Structurizr DSL

Purpose-built for C4 architecture diagrams.

**Why Rejected:**
- Only supports C4, not general UML
- Would require two tools (Structurizr + something for UML)
- Smaller ecosystem and tool support

### Alternative 3: Draw.io/Excalidraw

GUI-based diagramming tools.

**Why Rejected:**
- Binary/XML files don't diff well in git
- Harder to maintain as code evolves
- Requires specific tools to edit
- Time-consuming to keep updated

### Alternative 4: TikZ (Pure LaTeX)

Native LaTeX diagramming.

**Why Rejected:**
- Very steep learning curve
- Verbose syntax for simple diagrams
- No live preview workflow
- PlantUML outputs can be included in LaTeX anyway

## References

- [PlantUML Documentation](https://plantuml.com/)
- [C4-PlantUML Library](https://github.com/plantuml-stdlib/C4-PlantUML)
- [PlantUML VS Code Extension](https://marketplace.visualstudio.com/items?itemName=jebbs.plantuml)
- [PlantUML Themes](https://plantuml.com/theme)
