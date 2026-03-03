# claude-linux-desktop

> Native Linux desktop-app för Claude AI — med MCP, lokal agent-integration och Anthropic cowork-potential.

**Status:** Idé — 2026-03-03
**Källa:** Claude76 + Björn, session om armada-arkitektur

---

## Kärntanken

Anthropic har desktop-app för Mac och Windows. Linux saknas. Ingen officiell plan synlig.

Bygg det vi vet att Linux-communityn vill ha — och gör det på ett sätt som Anthropic *kan* adoptera eller samarbeta kring.

---

## Vad som skiljer den från en webbläsar-wrapper

1. **MCP-integration native** — konfigurera och kör MCP-servrar direkt i appen
2. **Lokal agent-sidebar** — armada-status, Self-entries, eskaleringar i realtid bredvid chatten
3. **System tray** — Claude alltid tillgänglig, en tangentbordsgenväg
4. **Filsystem-kontext** — dra in filer/mappar direkt, inte via copy-paste
5. **Offline-läge** — fallback till lokal modell (Ollama) när ingen uppkoppling
6. **Tight Linux-integration** — D-Bus, notifications, Wayland/X11

---

## Inloggning — dual mode

**Vem kan använda appen:**
- Alla med Claude-konto (Free, Pro, Team) — loggar in via claude.ai i inbäddad WebView
- Claude Code-användare — samma inloggning
- Developers — API-nyckel direkt

```
Välj vid start:
  [A] API-nyckel       ← developers, powerusers
  [B] Logga in med claude.ai  ← alla andra med Claude-konto
```

Mode B = ingen teknisk setup. Vem som helst som betalar för Claude idag och kör Linux kan använda appen direkt. Det är det som gör den till en produkt, inte ett dev tool.

Mode B öppnar också dörren för Anthropic att ge ett riktigt OAuth-flöde i ett samarbete.

---

## Teknikval

**Tauri** (Rust + WebView)
- ~10MB binär vs Electrons ~150MB
- Linux-communityn uppskattar det
- Rust-backend = säkert, snabbt, paketeras enkelt som .deb/.AppImage/.flatpak

---

## Spinn-potential

- Linux-communities (HN, Reddit/r/linux, lobste.rs) reagerar starkt
- Anthropic får argument för officiellt samarbete — kontakt finns via research-mailet
- Base76-varumärket = "den som bygger det Anthropic inte hinner"
- OSS → community driver vidare, Anthropic kan adoptera

---

## Risk

Anthropic lanserar Linux-app själva. Motargument: du har visat leveransförmåga i deras ekosystem.

---

## Nästa steg (när energi finns)

- [ ] Sätt upp Tauri-projekt på GitHub (base76-research-lab/claude-linux-desktop)
- [ ] Bygg MVP: chat-vy + MCP-konfiguration + system tray
- [ ] HN-post: "I built a Linux desktop app for Claude because Anthropic hasn't"
- [ ] Kontakta Anthropic med demo — föreslå cowork
