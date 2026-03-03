# UX Spec — claude-linux-desktop

## Färgpalett (Anthropic-kompatibel)

| Token | Hex | Användning |
|---|---|---|
| `--bg-dark` | `#1a1a1a` | Fönsterbakgrund (dark mode) |
| `--bg-light` | `#F5F0EB` | Fönsterbakgrund (light mode) |
| `--accent` | `#CC785C` | Coral — knappar, fokus, highlights |
| `--text-dark` | `#E8E3DD` | Text på mörk bakgrund |
| `--text-light` | `#2D2D2D` | Text på ljus bakgrund |
| `--bubble-user` | `#CC785C` | Användarens meddelandebubbla |
| `--bubble-claude` | `#F5F0EB` | Claudes meddelandebubbla |
| `--footer` | `#888888` | Footer-text |
| `--sidebar-bg` | `#242424` | Sidebar bakgrund |
| `--border` | `#333333` | Subtila separatorer |

---

## Fönsterstruktur

### Default (sidebar stängd)

```
┌─────────────────────────────────────────────────────────────┐
│  ● ○ ○   claude                              [sidebar ⇥]   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│                                                             │
│            ╭─────────────────────────────╮                 │
│            │  How can I help you today?  │  ← coral        │
│            ╰─────────────────────────────╯                 │
│                                                             │
│   ╭──────────────────────────────────────────────────╮     │
│   │ Claude                                           │     │
│   │ Sure! Here's what I found...                     │     │
│   ╰──────────────────────────────────────────────────╯     │
│                                                             │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│  [📎] [  Skriv ett meddelande...              ] [↑ skicka] │
├─────────────────────────────────────────────────────────────┤
│  Produced by Base76 Research Lab · github.com/base76-research-lab/claude-linux-desktop  │
└─────────────────────────────────────────────────────────────┘
```

### Med sidebar öppen (switch aktiverad)

```
┌───────────────────────────────┬─────────────────────────────┐
│  ● ○ ○   claude  [sidebar ⇤] │  MCP & Filer                │
├───────────────────────────────┼─────────────────────────────┤
│                               │  MCP-servrar                │
│   [chat-vy]                   │  ● conductor-graph-mcp  ✓   │
│                               │  ○ filesystem               │
│                               │  + Lägg till server...      │
│                               │                             │
│                               │  ─────────────────────────  │
│                               │  Filer i kontext            │
│                               │  + Dra hit eller klicka     │
│                               │                             │
│                               │  README.md          [×]     │
│                               │  server.py          [×]     │
│                               │                             │
├───────────────────────────────┴─────────────────────────────┤
│  [📎] [  Skriv ett meddelande...              ] [↑ skicka]  │
├─────────────────────────────────────────────────────────────┤
│  Produced by Base76 Research Lab · github.com/base76-research-lab/claude-linux-desktop  │
└─────────────────────────────────────────────────────────────┘
```

---

## Onboarding (första start)

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                    ◆  claude                                │
│                   för Linux                                 │
│                                                             │
│         Välj hur du vill logga in                           │
│                                                             │
│   ╭──────────────────────╮  ╭──────────────────────────╮   │
│   │  Logga in med        │  │  API-nyckel               │   │
│   │  claude.ai           │  │  (developers)             │   │
│   ╰──────────────────────╯  ╰──────────────────────────╯   │
│                                                             │
│         [Konfigurera MCP-servrar →]                         │
│         [Hoppa över, börja chatta →]                        │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│  Produced by Base76 Research Lab · github.com/...           │
└─────────────────────────────────────────────────────────────┘
```

---

## Komponenter

### Titlebar
- Fönsterkontroller (● ○ ○) vänster
- `claude` logotyp centrerat
- Sidebar-toggle höger: `[⇥]` stängd / `[⇤]` öppen

### Chat
- Användarbubbla: coral `#CC785C`, höger-alignad
- Claude-bubbla: warm white `#F5F0EB`, vänster-alignad, subtil border-radius
- Streaming-indikator: pulsande coral punkt

### Input
- Paperclip-ikon för filbilaga
- Fritext-fält, auto-expand
- Skicka-knapp coral vid aktiv text, grå vid tom

### Sidebar
- Bredd: 260px
- Två sektioner: MCP-servrar + Filer i kontext
- Toggle per MCP-server (on/off)
- Filer: chip-format med [×] för att ta bort

### Footer
- En rad, subtil text `#888888`
- `Produced by Base76 Research Lab · github.com/base76-research-lab/claude-linux-desktop`

---

## Sidebar default

**Stängd vid start.** Användaren öppnar vid behov via switch i titlebar.
