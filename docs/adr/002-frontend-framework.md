# ADR-002: Svelte as Frontend Framework

## Status

Accepted

## Date

2024-01-15

## Context

With Tauri as our desktop framework, we need to choose a frontend framework for building the user interface. The UI requirements include:

1. Complex data visualizations (heatmaps, timelines, statistics charts)
2. Interactive 2D map rendering for game positions
3. Responsive, modern design
4. Good TypeScript support
5. Manageable bundle size
6. Developer experience / productivity

The main options considered:
- **React**: Most popular, largest ecosystem
- **Vue**: Good balance of features and simplicity
- **Svelte**: Compiler-based, minimal runtime
- **Solid**: React-like but with fine-grained reactivity

## Decision

We will use **Svelte 5** with TypeScript as our frontend framework.

## Consequences

### Positive

1. **Minimal Boilerplate**: Svelte's syntax is more concise than React. Components are simpler to write and read.

   ```svelte
   <!-- Svelte: reactive by default -->
   <script>
     let count = 0;
   </script>
   <button on:click={() => count++}>{count}</button>
   
   <!-- vs React: explicit state management -->
   const [count, setCount] = useState(0);
   <button onClick={() => setCount(c => c + 1)}>{count}</button>
   ```

2. **Excellent Performance**: Svelte compiles to vanilla JavaScript with no virtual DOM. This results in smaller bundles and faster runtime performance.

3. **Built-in Features**: Stores, transitions, and animations are built-in, reducing dependency count.

4. **Tauri Integration**: Official Tauri examples and templates use Svelte. The Tauri team actively supports Svelte integration.

5. **Svelte 5 Runes**: The new runes system provides fine-grained reactivity while maintaining simplicity.

6. **Growing Ecosystem**: While smaller than React, the Svelte ecosystem has mature solutions for our needs (charting, UI components).

### Negative

1. **Smaller Ecosystem**: Fewer third-party components compared to React. May need to build more custom solutions.

2. **Smaller Talent Pool**: Fewer developers know Svelte compared to React. Could be a consideration if the team grows.

3. **Breaking Changes**: Svelte 5 is relatively new; some ecosystem libraries may not be updated yet.

4. **Less Enterprise Adoption**: Fewer case studies and enterprise patterns to reference.

### Neutral

1. **Learning Curve**: While different from React/Vue, Svelte is generally considered easy to learn. Trade-off is acceptable.

2. **Tooling**: VS Code support via Svelte extension is excellent. Comparable to other frameworks.

## Alternatives Considered

### Alternative 1: React

The safe, conventional choice with the largest ecosystem.

**Why Rejected:**
- More boilerplate for the same functionality
- Larger bundle size impact
- Virtual DOM overhead (minor, but unnecessary for our use case)
- Ecosystem size is less relevant for a desktop app with specific needs

### Alternative 2: Vue

Good middle ground between React and Svelte.

**Why Rejected:**
- Options API vs Composition API complexity
- Slightly more verbose than Svelte
- No significant advantage over Svelte for our use case
- Svelte's Tauri integration is more actively maintained

### Alternative 3: Solid

Similar reactive model to Svelte with React-like JSX.

**Why Rejected:**
- Even smaller ecosystem than Svelte
- Less mature Tauri integration
- JSX adds complexity without clear benefit

## UI Component Strategy

Given the smaller Svelte ecosystem, we'll use:

1. **shadcn-svelte**: Port of shadcn/ui providing accessible, customizable components
2. **TailwindCSS**: Utility-first CSS framework
3. **Pixi.js**: For 2D canvas rendering (framework-agnostic, works well with Svelte)
4. **Chart.js** or **Apache ECharts**: For statistics visualization

## References

- [Svelte 5 Documentation](https://svelte.dev/docs)
- [Svelte vs React Performance](https://krausest.github.io/js-framework-benchmark/)
- [Tauri + Svelte Template](https://github.com/tauri-apps/tauri/tree/dev/examples/sveltekit)
- [shadcn-svelte](https://www.shadcn-svelte.com/)
