# LaTeX Documentation Build

This directory contains the LaTeX source for generating the OutOf5K technical documentation PDF.

## Quick Start

From the `docs/` directory, use the build script:

```bash
# Generate diagrams and build PDF
./build.sh all

# Or just diagrams
./build.sh diagrams

# Or use make from this directory
make all
```

The build script will automatically download PlantUML if needed.

## Prerequisites

### Java (Required for PlantUML)

```bash
# macOS
brew install openjdk

# Ubuntu/Debian
sudo apt install default-jdk

# Windows
choco install openjdk
```

### TeX Distribution (Required for PDF)

**macOS:**
```bash
brew install --cask mactex
# Or minimal version:
brew install --cask basictex
```

**Ubuntu/Debian:**
```bash
sudo apt install texlive-full
# Or minimal:
sudo apt install texlive-base texlive-latex-recommended texlive-fonts-recommended texlive-xetex
```

**Windows:**
- Download and install [MiKTeX](https://miktex.org/download)

### Required LaTeX Packages

The following packages are required (installed automatically with texlive-full):
- tufte-book
- fontspec (for XeLaTeX)
- listings
- graphicx
- hyperref
- booktabs
- xcolor
- microtype

## Build Commands

From the `docs/` directory:

```bash
./build.sh diagrams    # Generate PNG diagrams from PlantUML
./build.sh svg         # Generate SVG diagrams (for web/markdown)
./build.sh pdf         # Build PDF (assumes diagrams exist)
./build.sh all         # Generate diagrams and build PDF
./build.sh clean       # Remove LaTeX auxiliary files
./build.sh distclean   # Remove all generated files including PlantUML
./build.sh help        # Show help
```

Or from this directory using Make:

```bash
make diagrams
make pdf
make all
make clean
```

## Output

| Output | Location |
|--------|----------|
| PDF | `docs/latex/main.pdf` |
| PNG Diagrams | `docs/latex/figures/` |
| PlantUML JAR | `docs/.plantuml/plantuml.jar` (auto-downloaded) |

## Project Structure

```
latex/
├── main.tex              # Master document
├── Makefile              # Build shortcuts
├── README.md             # This file
├── chapters/
│   ├── 01-introduction.tex
│   ├── 02-architecture.tex
│   ├── 03-data-flow.tex
│   ├── 04-patterns.tex
│   ├── 05-decisions.tex
│   ├── 06-development.tex
│   └── appendix-a-schema.tex
└── figures/              # Generated diagrams (gitignored)
```

## Troubleshooting

### Java not found

The build script will tell you how to install Java for your platform.

### Missing fonts (XeLaTeX)

If you see font errors with XeLaTeX, you can:

1. Install the required fonts (Palatino, Helvetica, Menlo)
2. Or edit `main.tex` to use different fonts
3. Or switch to pdflatex (comment out `\usepackage{fontspec}` lines)

### Missing LaTeX packages

**TeX Live:**
```bash
tlmgr install tufte-latex
```

**MiKTeX:** Will prompt to install automatically.

### PlantUML download fails

If automatic download fails, manually download:
```bash
mkdir -p docs/.plantuml
curl -L -o docs/.plantuml/plantuml.jar \
  https://github.com/plantuml/plantuml/releases/download/v1.2024.0/plantuml-1.2024.0.jar
```

## Customization

### Adding new chapters

1. Create `chapters/XX-name.tex`
2. Add `\input{chapters/XX-name}` to `main.tex`

### Changing fonts

Edit the font definitions in `main.tex`:
```latex
\setmainfont{Your Font}
\setsansfont{Your Sans Font}
\setmonofont{Your Mono Font}[Scale=0.85]
```

### Using pdflatex instead of XeLaTeX

Comment out fontspec in `main.tex`:
```latex
% \usepackage{fontspec}
% \setmainfont{...}
```
