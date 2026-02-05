#!/bin/bash
# OutOf5K Documentation Build Script
# Handles PlantUML download and diagram generation

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLANTUML_VERSION="1.2024.0"
PLANTUML_JAR="$SCRIPT_DIR/.plantuml/plantuml.jar"
PLANTUML_URL="https://github.com/plantuml/plantuml/releases/download/v${PLANTUML_VERSION}/plantuml-${PLANTUML_VERSION}.jar"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Check for Java
check_java() {
    if ! command -v java &> /dev/null; then
        print_error "Java is required but not installed."
        echo "  Install Java:"
        echo "    macOS:  brew install openjdk"
        echo "    Ubuntu: sudo apt install default-jdk"
        echo "    Windows: choco install openjdk"
        exit 1
    fi
    print_status "Java found: $(java -version 2>&1 | head -n 1)"
}

# Download PlantUML if not present
download_plantuml() {
    if [ -f "$PLANTUML_JAR" ]; then
        print_status "PlantUML already downloaded"
        return 0
    fi

    print_status "Downloading PlantUML v${PLANTUML_VERSION}..."
    mkdir -p "$SCRIPT_DIR/.plantuml"
    
    if command -v curl &> /dev/null; then
        curl -L -o "$PLANTUML_JAR" "$PLANTUML_URL"
    elif command -v wget &> /dev/null; then
        wget -O "$PLANTUML_JAR" "$PLANTUML_URL"
    else
        print_error "Neither curl nor wget found. Please install one."
        exit 1
    fi
    
    print_status "PlantUML downloaded to $PLANTUML_JAR"
}

# Generate diagrams from PlantUML files
generate_diagrams() {
    local output_dir="$SCRIPT_DIR/latex/figures"
    local arch_dir="$SCRIPT_DIR/architecture"
    
    print_status "Generating diagrams..."
    
    # Create output directories
    mkdir -p "$output_dir/c4"
    mkdir -p "$output_dir/uml/sequence"
    mkdir -p "$output_dir/uml/class"
    mkdir -p "$output_dir/uml/state"
    
    # Generate C4 diagrams
    if ls "$arch_dir/c4/"*.puml 1> /dev/null 2>&1; then
        print_status "Generating C4 diagrams..."
        for file in "$arch_dir/c4/"*.puml; do
            filename=$(basename "$file" .puml)
            java -jar "$PLANTUML_JAR" -tpng -o "$output_dir/c4" "$file"
            print_status "  Generated: $filename.png"
        done
    fi
    
    # Generate sequence diagrams
    if ls "$arch_dir/uml/sequence/"*.puml 1> /dev/null 2>&1; then
        print_status "Generating sequence diagrams..."
        for file in "$arch_dir/uml/sequence/"*.puml; do
            filename=$(basename "$file" .puml)
            java -jar "$PLANTUML_JAR" -tpng -o "$output_dir/uml/sequence" "$file"
            print_status "  Generated: $filename.png"
        done
    fi
    
    # Generate class diagrams
    if ls "$arch_dir/uml/class/"*.puml 1> /dev/null 2>&1; then
        print_status "Generating class diagrams..."
        for file in "$arch_dir/uml/class/"*.puml; do
            filename=$(basename "$file" .puml)
            java -jar "$PLANTUML_JAR" -tpng -o "$output_dir/uml/class" "$file"
            print_status "  Generated: $filename.png"
        done
    fi
    
    # Generate state diagrams
    if ls "$arch_dir/uml/state/"*.puml 1> /dev/null 2>&1; then
        print_status "Generating state diagrams..."
        for file in "$arch_dir/uml/state/"*.puml; do
            filename=$(basename "$file" .puml)
            java -jar "$PLANTUML_JAR" -tpng -o "$output_dir/uml/state" "$file"
            print_status "  Generated: $filename.png"
        done
    fi
    
    print_status "All diagrams generated in $output_dir"
}

# Generate SVG versions (useful for web/markdown)
generate_svg() {
    local output_dir="$SCRIPT_DIR/latex/figures"
    local arch_dir="$SCRIPT_DIR/architecture"
    
    print_status "Generating SVG versions..."
    
    # Find all .puml files and generate SVGs
    find "$arch_dir" -name "*.puml" | while read -r file; do
        local rel_path="${file#$arch_dir/}"
        local dir_path=$(dirname "$rel_path")
        local filename=$(basename "$file" .puml)
        
        mkdir -p "$output_dir/$dir_path"
        java -jar "$PLANTUML_JAR" -tsvg -o "$output_dir/$dir_path" "$file"
    done
    
    print_status "SVG diagrams generated"
}

# Build PDF with LaTeX
build_pdf() {
    local latex_dir="$SCRIPT_DIR/latex"
    
    if ! command -v xelatex &> /dev/null; then
        print_warning "XeLaTeX not found. Trying pdflatex..."
        if ! command -v pdflatex &> /dev/null; then
            print_error "No LaTeX compiler found. Install TeX Live or MiKTeX."
            echo "  macOS:  brew install --cask mactex"
            echo "  Ubuntu: sudo apt install texlive-full"
            exit 1
        fi
        LATEX_CMD="pdflatex"
    else
        LATEX_CMD="xelatex"
    fi
    
    print_status "Building PDF with $LATEX_CMD..."
    cd "$latex_dir"
    
    # Run twice for references
    $LATEX_CMD -interaction=nonstopmode main.tex > /dev/null 2>&1 || true
    $LATEX_CMD -interaction=nonstopmode main.tex
    
    if [ -f "main.pdf" ]; then
        print_status "PDF generated: $latex_dir/main.pdf"
    else
        print_error "PDF generation failed"
        exit 1
    fi
}

# Clean generated files
clean() {
    print_status "Cleaning generated files..."
    rm -rf "$SCRIPT_DIR/latex/figures"
    rm -f "$SCRIPT_DIR/latex/"*.aux
    rm -f "$SCRIPT_DIR/latex/"*.log
    rm -f "$SCRIPT_DIR/latex/"*.out
    rm -f "$SCRIPT_DIR/latex/"*.toc
    rm -f "$SCRIPT_DIR/latex/"*.lof
    rm -f "$SCRIPT_DIR/latex/"*.lot
    print_status "Clean complete"
}

# Deep clean including PlantUML and PDF
distclean() {
    clean
    print_status "Removing PlantUML and PDF..."
    rm -rf "$SCRIPT_DIR/.plantuml"
    rm -f "$SCRIPT_DIR/latex/main.pdf"
    print_status "Distclean complete"
}

# Show help
show_help() {
    echo "OutOf5K Documentation Build Script"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  diagrams    Generate PNG diagrams from PlantUML files"
    echo "  svg         Generate SVG diagrams (for web use)"
    echo "  pdf         Build PDF documentation (requires LaTeX)"
    echo "  all         Generate diagrams and build PDF"
    echo "  clean       Remove generated files (keep PlantUML)"
    echo "  distclean   Remove all generated files including PlantUML"
    echo "  help        Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 diagrams    # Generate just the diagrams"
    echo "  $0 all         # Generate diagrams and PDF"
    echo "  $0 clean       # Clean up"
}

# Main
main() {
    local command="${1:-help}"
    
    case "$command" in
        diagrams)
            check_java
            download_plantuml
            generate_diagrams
            ;;
        svg)
            check_java
            download_plantuml
            generate_svg
            ;;
        pdf)
            build_pdf
            ;;
        all)
            check_java
            download_plantuml
            generate_diagrams
            build_pdf
            ;;
        clean)
            clean
            ;;
        distclean)
            distclean
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
