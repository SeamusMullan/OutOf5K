# ADR-006: Phased 3D Replay Implementation

## Status

Accepted

## Date

2024-01-15

## Context

A key feature request for OutOf5K is the ability to replay demos in 3D, similar to CSGO Demos Manager. This would allow players to review their gameplay from any angle, including first-person perspective.

The technical challenges are significant:
- CS2 map geometry must be extracted or recreated
- Player models have licensing concerns
- Real-time 3D rendering in a web context has performance implications
- The scope could easily consume the entire development timeline

We need to balance the user's desire for rich 3D replay with practical development constraints.

## Decision

We will implement a **phased approach** to demo visualization:

1. **Phase 1 (MVP)**: 2D radar-style map visualization
2. **Phase 2**: Enhanced 2D with utility trajectories, player paths
3. **Phase 4**: 3D replay viewer (scope TBD based on learnings)

## Consequences

### Positive

1. **Ship Faster**: 2D visualization is achievable in the MVP timeline. Users get value sooner.

2. **Validate Core Features**: Test the demo parsing, statistics, and analysis features before investing in 3D.

3. **Learn User Needs**: Real user feedback will inform what 3D features are actually important.

4. **Reduce Risk**: The biggest technical unknowns (map extraction, model licensing) are deferred.

5. **2D Still Valuable**: Professional CS2 analysts often use 2D views. It's not a compromise for MVP.

6. **Incremental Investment**: Each phase builds on the previous, no wasted work.

### Negative

1. **Not Full-Featured Initially**: Users wanting CSGO Demos Manager-style 3D won't get it in v1.

2. **Marketing Challenge**: "2D first" is less exciting than "full 3D replay."

3. **Potential Rework**: The 2D architecture may need changes to support 3D.

### Neutral

1. **Competition**: Other tools exist for 3D replay. We differentiate on analysis features instead.

## Phase Details

### Phase 1: 2D Radar Map (MVP)

**Features:**
- Overhead map view (using radar images)
- Player position indicators
- Real-time position updates during playback
- Round-by-round timeline
- Basic event markers (kills, bomb plant)

**Technology:**
- Pixi.js for high-performance 2D canvas rendering
- Map radar images extracted from CS2 game files
- Position data from demo parsing

**Example:**
```
    ┌─────────────────────────────────────┐
    │               A SITE                │
    │           ●CT1    ●CT2              │
    │                                     │
    │      ●T1                            │
    │          ●T2      ●T3               │
    │                                     │
    │               B SITE                │
    │           ●CT3    ●CT4    ●CT5      │
    └─────────────────────────────────────┘
```

### Phase 2: Enhanced 2D Analysis

**Features:**
- Player movement paths over time
- Grenade trajectories and landing spots
- Smoke/molotov coverage areas
- Kill lines connecting attacker → victim
- Heatmaps for positioning patterns
- Crosshair placement tracking

**Technology:**
- Same Pixi.js foundation
- Additional rendering layers
- Animation system for playback

### Phase 4: 3D Replay

**Research Questions:**
- Map geometry: Extract from game files or recreate?
- Player models: Stylized/abstract vs realistic?
- Rendering: Three.js, Babylon.js, or embedded game engine?
- Performance: Target frame rate, level of detail

**Potential Approaches:**

**Option A: Three.js Web-Based**
- Pros: Web tech, integrates with Tauri webview
- Cons: Map extraction is complex, performance concerns

**Option B: Stylized/Abstract 3D**
- Pros: Avoid model licensing, unique visual style
- Cons: Less immersive, players may prefer realistic

**Option C: Launch CS2**
- Pros: Perfect fidelity, Valve handles everything
- Cons: Requires CS2 installed, leaves our app context

**Option D: Embedded Game Engine**
- Pros: High performance, proven 3D capabilities
- Cons: Complex integration, large bundle size

**Decision Deferred**: We'll make this decision after Phase 2, informed by user feedback and technical research.

## Success Criteria for Each Phase

### Phase 1 Success (Gate to Phase 2)
- [ ] 2D map renders correctly for all competitive maps
- [ ] Position playback is smooth (60fps)
- [ ] Users can effectively review rounds
- [ ] Positive user feedback on core utility

### Phase 2 Success (Gate to Phase 4)
- [ ] Path visualization helps identify positioning mistakes
- [ ] Utility analysis provides actionable insights
- [ ] Users request 3D features specifically
- [ ] Technical foundation is solid

### Phase 4 Go/No-Go Criteria
- User demand: Do users actually want 3D or is 2D sufficient?
- Technical feasibility: Have we solved map extraction?
- Resource availability: Do we have capacity for the investment?
- Competitive landscape: Have other tools solved this better?

## Alternatives Considered

### Alternative 1: 3D From Day One

Build full 3D replay as core feature.

**Why Rejected:**
- Would delay MVP by 6+ months
- High technical risk with uncertain solutions
- May not be what users actually need most

### Alternative 2: Skip 3D Entirely

Focus only on statistics and 2D analysis.

**Why Rejected:**
- 3D replay is a clear user desire
- Limits long-term differentiation
- Leaves opportunity for competitors

### Alternative 3: Integration with Existing 3D Tools

Partner with or embed existing replay tools.

**Why Rejected:**
- No suitable open-source options exist
- Commercial tools won't want to be embedded
- Dependency on external tool maintenance

## References

- [CSGO Demos Manager](https://github.com/akiver/CSGO-Demos-Manager) - Prior art
- [Three.js Documentation](https://threejs.org/docs/)
- [Source 2 SDK Resources](https://developer.valvesoftware.com/wiki/Source_2)
- [Pixi.js Performance](https://pixijs.com/guides/production/performance-tips)
