<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { getVersion } from '@tauri-apps/api/app';
  import { listen } from '@tauri-apps/api/event';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  let appVersion = $state('...');
  let isActive = $state(false);
  let lastStrafe = $state('—');
  let capturingField = $state<string | null>(null);
  let saveStatus = $state('');

  let settings = $state({
    key_left: 'KeyA',
    key_right: 'KeyD',
    key_forward: 'KeyW',
    key_back: 'KeyS',
    delay_min_ms: 70,
    delay_max_ms: 89,
    strafe_forward_back: false,
    start_active: false,
    start_minimized: false,
  });

  onMount(async () => {
    appVersion = await getVersion().catch(() => '?');
    const [loadedSettings, active] = await Promise.all([
      invoke<typeof settings>('get_settings'),
      invoke<boolean>('get_active'),
    ]);
    settings = loadedSettings;
    isActive = active;
    await listen<boolean>('active-changed', (e) => (isActive = e.payload));
    await listen<string>('counter-strafe', (e) => (lastStrafe = e.payload));
  });

  async function toggleActive() {
    await invoke('set_hook_active', { active: !isActive });
  }

  async function saveSettings() {
    try {
      await invoke('update_settings', { newSettings: settings });
      saveStatus = 'Saved';
      setTimeout(() => (saveStatus = ''), 2000);
    } catch (e) {
      saveStatus = 'Error: ' + e;
    }
  }

  function startCapture(field: string) {
    capturingField = field;
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (!capturingField) return;
    event.preventDefault();
    const key = mapBrowserKeyToRdev(event.code);
    if (!key) return;
    (settings as Record<string, unknown>)[capturingField] = key;
    capturingField = null;
  }

  function mapBrowserKeyToRdev(code: string): string | null {
    const map: Record<string, string> = {
      KeyA: 'KeyA', KeyB: 'KeyB', KeyC: 'KeyC', KeyD: 'KeyD', KeyE: 'KeyE',
      KeyF: 'KeyF', KeyG: 'KeyG', KeyH: 'KeyH', KeyI: 'KeyI', KeyJ: 'KeyJ',
      KeyK: 'KeyK', KeyL: 'KeyL', KeyM: 'KeyM', KeyN: 'KeyN', KeyO: 'KeyO',
      KeyP: 'KeyP', KeyQ: 'KeyQ', KeyR: 'KeyR', KeyS: 'KeyS', KeyT: 'KeyT',
      KeyU: 'KeyU', KeyV: 'KeyV', KeyW: 'KeyW', KeyX: 'KeyX', KeyY: 'KeyY',
      KeyZ: 'KeyZ',
      Digit1: 'Num1', Digit2: 'Num2', Digit3: 'Num3', Digit4: 'Num4',
      Digit5: 'Num5', Digit6: 'Num6', Digit7: 'Num7', Digit8: 'Num8',
      Digit9: 'Num9', Digit0: 'Num0',
      F1: 'F1', F2: 'F2', F3: 'F3', F4: 'F4', F5: 'F5', F6: 'F6',
      F7: 'F7', F8: 'F8', F9: 'F9', F10: 'F10', F11: 'F11', F12: 'F12',
      Space: 'Space', Tab: 'Tab', Escape: 'Escape', Enter: 'Return',
      Backspace: 'Backspace', Delete: 'Delete',
      ArrowLeft: 'LeftArrow', ArrowRight: 'RightArrow',
      ArrowUp: 'UpArrow', ArrowDown: 'DownArrow',
      ShiftLeft: 'ShiftLeft', ShiftRight: 'ShiftRight',
      ControlLeft: 'ControlLeft', ControlRight: 'ControlRight',
      AltLeft: 'Alt', AltRight: 'AltGr',
      CapsLock: 'CapsLock', Insert: 'Insert', Home: 'Home', End: 'End',
      PageUp: 'PageUp', PageDown: 'PageDown',
      Backquote: 'BackQuote', Minus: 'Minus', Equal: 'Equal',
      BracketLeft: 'LeftBracket', BracketRight: 'RightBracket',
      Semicolon: 'SemiColon', Quote: 'Quote', Backslash: 'BackSlash',
      Comma: 'Comma', Period: 'Dot', Slash: 'Slash',
    };
    return map[code] ?? null;
  }

  function keyLabel(key: string): string {
    const labels: Record<string, string> = {
      KeyA: 'A', KeyB: 'B', KeyC: 'C', KeyD: 'D', KeyE: 'E',
      KeyF: 'F', KeyG: 'G', KeyH: 'H', KeyI: 'I', KeyJ: 'J',
      KeyK: 'K', KeyL: 'L', KeyM: 'M', KeyN: 'N', KeyO: 'O',
      KeyP: 'P', KeyQ: 'Q', KeyR: 'R', KeyS: 'S', KeyT: 'T',
      KeyU: 'U', KeyV: 'V', KeyW: 'W', KeyX: 'X', KeyY: 'Y', KeyZ: 'Z',
      Space: 'Space', Tab: 'Tab', Escape: 'Esc', Return: 'Enter',
      ShiftLeft: 'L.Shift', ShiftRight: 'R.Shift',
      ControlLeft: 'L.Ctrl', ControlRight: 'R.Ctrl',
      Alt: 'L.Alt', AltGr: 'R.Alt',
      LeftArrow: 'Left', RightArrow: 'Right', UpArrow: 'Up', DownArrow: 'Down',
    };
    return labels[key] ?? key;
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if capturingField}
  <div class="capture-overlay">
    <div class="capture-dialog">
      <div class="capture-title">PRESS A KEY</div>
      <div class="capture-hint">Listening for input…</div>
      <button class="btn-ghost" onclick={() => (capturingField = null)}>Cancel</button>
    </div>
  </div>
{/if}

<main class="app">
  <header class="app-header">
    <div class="header-accent"></div>
    <div class="header-content">
      <div class="app-title">
        <span class="title-main">CS2</span>
        <span class="title-sub">TAPPING</span>
      </div>
      <div class="header-right">
        <span class="version">v{appVersion}</span>
      </div>
    </div>
  </header>

  <div class="content">
    <section class="tile tile-status" class:tile-active={isActive}>
      <div class="tile-label">STATUS</div>
      <div class="status-text">{isActive ? 'ACTIVE' : 'INACTIVE'}</div>
      <div class="status-hint">{isActive ? 'Counter-strafe hook running' : 'Hook is disabled — click to enable'}</div>
      <button class="btn-toggle" class:btn-toggle-on={isActive} onclick={toggleActive}>
        {isActive ? 'DISABLE' : 'ENABLE'}
      </button>
    </section>

    <section class="tile tile-keys">
      <div class="tile-label">MOVEMENT KEYS</div>
      <div class="keys-grid">
        {#each (['key_left', 'key_right'] as const) as field}
          {@const label = field === 'key_left' ? 'Left' : 'Right'}
          <div class="key-row">
            <span class="key-name">{label}</span>
            <button
              class="key-btn"
              class:key-btn-capturing={capturingField === field}
              onclick={() => startCapture(field)}
            >
              {keyLabel((settings as unknown as Record<string, string>)[field])}
            </button>
          </div>
        {/each}
      </div>
      <div class="keys-divider">
        <div class="toggle-row" onclick={() => (settings.strafe_forward_back = !settings.strafe_forward_back)}>
          <span class="toggle-label">Enable Forward / Back ?</span>
          <button class="toggle-pill" class:toggle-pill-on={settings.strafe_forward_back}><span class="toggle-thumb"></span></button>
        </div>
      </div>
      <div class="keys-grid" class:keys-grid-disabled={!settings.strafe_forward_back}>
        {#each (['key_forward', 'key_back'] as const) as field}
          {@const label = field === 'key_forward' ? 'Forward' : 'Back'}
          <div class="key-row">
            <span class="key-name">{label}</span>
            <button
              class="key-btn"
              class:key-btn-capturing={capturingField === field}
              disabled={!settings.strafe_forward_back}
              onclick={() => startCapture(field)}
            >
              {keyLabel((settings as unknown as Record<string, string>)[field])}
            </button>
          </div>
        {/each}
      </div>
    </section>

    <section class="tile tile-timing">
      <div class="tile-label">TIMING (ms)</div>
      <div class="timing-row">
        <label class="timing-label">Min delay</label>
        <input class="timing-input" type="number" min="10" max="500" bind:value={settings.delay_min_ms} />
      </div>
      <div class="timing-row">
        <label class="timing-label">Max delay</label>
        <input class="timing-input" type="number" min="10" max="500" bind:value={settings.delay_max_ms} />
      </div>
      <div class="strafe-indicator">Last strafe: <span class="strafe-key">{lastStrafe}</span></div>
    </section>

    <div class="save-row">
      <button class="btn-save" onclick={saveSettings}>APPLY</button>
      {#if saveStatus}
        <span class="save-status" class:save-error={saveStatus.startsWith('Error')}>{saveStatus}</span>
      {/if}
    </div>
  </div>

  <footer class="app-footer">
    <button class="btn-footer-settings" onclick={() => goto('/settings')}>⚙ SETTINGS</button>
    <span>CS2 Tapping by Dmitry Osin &lt;d@osin.pro&gt;</span>
  </footer>
</main>

<style>
  :global(*) { box-sizing: border-box; margin: 0; padding: 0; }
  :global(body) {
    background: #1a1a1a; color: #e8e8e8;
    font-family: 'Segoe UI', system-ui, sans-serif; font-size: 13px;
    user-select: none; -webkit-user-select: none; overflow: hidden;
  }
  .app { display: flex; flex-direction: column; height: 100vh; }
  .app-header { flex-shrink: 0; background: #111; border-bottom: 1px solid #2a2a2a; }
  .header-accent { height: 3px; background: #e63946; }
  .header-content { display: flex; justify-content: space-between; align-items: center; padding: 10px 16px; }
  .app-title { display: flex; align-items: baseline; gap: 6px; }
  .title-main { font-size: 20px; font-weight: 700; letter-spacing: 2px; color: #e63946; }
  .title-sub { font-size: 14px; font-weight: 300; letter-spacing: 4px; color: #888; text-transform: uppercase; }
  .header-right { display: flex; align-items: center; gap: 12px; }
  .version { color: #555; font-size: 11px; }
  .content { flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 12px; }
  .tile { background: #222; border: 1px solid #2e2e2e; padding: 14px 16px; }
  .tile-label { font-size: 10px; font-weight: 700; letter-spacing: 2px; color: #555; margin-bottom: 10px; }
  .tile-status { border-left: 3px solid #444; transition: border-color 0.2s; }
  .tile-active { border-left-color: #e63946; }
  .status-text { font-size: 28px; font-weight: 700; letter-spacing: 3px; line-height: 1; margin-bottom: 4px; }
  .tile-active .status-text { color: #e63946; }
  .status-hint { font-size: 11px; color: #666; margin-bottom: 12px; }
  .btn-toggle { padding: 8px 24px; border: 1px solid #555; background: transparent; color: #ccc; font-size: 11px; font-weight: 700; letter-spacing: 2px; cursor: pointer; transition: all 0.15s; }
  .btn-toggle:hover { border-color: #e63946; color: #e63946; }
  .btn-toggle-on { background: #e63946 !important; border-color: #e63946 !important; color: #fff !important; }
  .btn-toggle-on:hover { background: #c1121f !important; }
  .keys-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; }
  .key-row { display: flex; align-items: center; gap: 8px; }
  .key-name { width: 52px; font-size: 11px; color: #666; text-transform: uppercase; letter-spacing: 1px; }
  .key-btn { flex: 1; padding: 6px 10px; background: #2a2a2a; border: 1px solid #3a3a3a; color: #ddd; font-size: 12px; font-weight: 600; cursor: pointer; text-align: center; transition: all 0.15s; }
  .key-btn:hover { background: #333; border-color: #e63946; color: #e63946; }
  .key-btn-capturing { border-color: #e63946 !important; color: #e63946 !important; animation: pulse 1s infinite; }
  @keyframes pulse { 0%,100%{opacity:1} 50%{opacity:0.5} }
  .keys-divider { margin: 10px 0 8px; border-top: 1px solid #2e2e2e; padding-top: 8px; }
  .keys-grid-disabled { opacity: 0.35; pointer-events: none; }
  .key-btn:disabled { cursor: default; }
  .toggle-row { display: flex; align-items: center; justify-content: space-between; padding: 6px 0; cursor: pointer; }
  .toggle-label { font-size: 12px; color: #aaa; }
  .toggle-pill { position: relative; width: 40px; height: 22px; background: #3a3a3a; border: 1px solid #4a4a4a; border-radius: 0; cursor: pointer; transition: background 0.2s; flex-shrink: 0; pointer-events: none; }
  .toggle-pill-on { background: #e63946 !important; border-color: #e63946 !important; }
  .toggle-thumb { position: absolute; top: 3px; left: 3px; width: 14px; height: 14px; background: #888; transition: transform 0.2s, background 0.2s; }
  .toggle-pill-on .toggle-thumb { transform: translateX(18px); background: #fff; }
  .timing-row { display: flex; align-items: center; gap: 10px; margin-bottom: 8px; }
  .timing-label { width: 80px; font-size: 11px; color: #666; }
  .timing-input { width: 80px; padding: 5px 8px; background: #2a2a2a; border: 1px solid #3a3a3a; color: #ddd; font-size: 13px; outline: none; transition: border-color 0.15s; }
  .timing-input:focus { border-color: #e63946; }
  .strafe-indicator { font-size: 11px; color: #555; margin-top: 4px; }
  .strafe-key { color: #e63946; font-weight: 600; }
  .save-row { display: flex; align-items: center; gap: 12px; }
  .btn-save { padding: 9px 28px; background: #e63946; border: none; color: #fff; font-size: 11px; font-weight: 700; letter-spacing: 2px; cursor: pointer; transition: background 0.15s; }
  .btn-save:hover { background: #c1121f; }
  .save-status { font-size: 11px; color: #4caf50; letter-spacing: 1px; }
  .save-error { color: #e63946; }
  .capture-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.75); display: flex; align-items: center; justify-content: center; z-index: 100; }
  .capture-dialog { background: #1a1a1a; border: 1px solid #e63946; padding: 32px 40px; text-align: center; }
  .capture-title { font-size: 16px; font-weight: 700; letter-spacing: 3px; color: #e63946; margin-bottom: 8px; }
  .capture-hint { font-size: 12px; color: #666; margin-bottom: 20px; }
  .btn-ghost { padding: 6px 20px; background: transparent; border: 1px solid #555; color: #888; font-size: 11px; cursor: pointer; letter-spacing: 1px; }
  .btn-ghost:hover { border-color: #e63946; color: #e63946; }
  .app-footer { flex-shrink: 0; padding: 8px 16px; font-size: 10px; color: #3a3a3a; border-top: 1px solid #222; display: flex; align-items: center; justify-content: space-between; }
  .btn-footer-settings { background: transparent; border: none; color: #555; font-size: 11px; font-weight: 700; letter-spacing: 1px; cursor: pointer; padding: 2px 0; transition: color 0.15s; }
  .btn-footer-settings:hover { color: #e63946; }
  .content::-webkit-scrollbar { width: 3px; }
  .content::-webkit-scrollbar-track { background: transparent; }
  .content::-webkit-scrollbar-thumb { background: #333; }
  .content::-webkit-scrollbar-thumb:hover { background: #e63946; }
</style>
