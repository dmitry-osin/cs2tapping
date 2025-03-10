<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  let autostart = $state(false);
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
    const [loadedSettings, autostartEnabled] = await Promise.all([
      invoke<typeof settings>('get_settings'),
      isEnabled().catch(() => false),
    ]);
    settings = loadedSettings;
    autostart = autostartEnabled;
  });

  async function saveSettings() {
    try {
      await invoke('update_settings', { newSettings: settings });
      saveStatus = 'Saved';
      setTimeout(() => (saveStatus = ''), 2000);
    } catch (e) {
      saveStatus = 'Error: ' + e;
    }
  }

  async function toggleAutostart() {
    autostart ? await disable() : await enable();
    autostart = !autostart;
  }
</script>

<main class="app">
  <header class="app-header">
    <div class="header-accent"></div>
    <div class="header-content">
      <div class="app-title">
        <span class="title-main">CS2</span>
        <span class="title-sub">TAPPING</span>
      </div>
      <div class="header-right">
        <button class="btn-back" onclick={() => goto('/')}>← BACK</button>
      </div>
    </div>
  </header>

  <div class="content">
    <section class="tile tile-startup">
      <div class="tile-label">STARTUP</div>
      <div class="toggle-row" onclick={toggleAutostart}>
        <span class="toggle-label">Start with Windows</span>
        <button class="toggle-pill" class:toggle-pill-on={autostart}><span class="toggle-thumb"></span></button>
      </div>
      <div class="toggle-row" onclick={() => (settings.start_active = !settings.start_active)}>
        <span class="toggle-label">Enable hook on start</span>
        <button class="toggle-pill" class:toggle-pill-on={settings.start_active}><span class="toggle-thumb"></span></button>
      </div>
      <div class="toggle-row" onclick={() => (settings.start_minimized = !settings.start_minimized)}>
        <span class="toggle-label">Start minimized to tray</span>
        <button class="toggle-pill" class:toggle-pill-on={settings.start_minimized}><span class="toggle-thumb"></span></button>
      </div>
    </section>

    <div class="save-row">
      <button class="btn-save" onclick={saveSettings}>SAVE SETTINGS</button>
      {#if saveStatus}
        <span class="save-status" class:save-error={saveStatus.startsWith('Error')}>{saveStatus}</span>
      {/if}
    </div>
  </div>

  <footer class="app-footer">CS2 Tapping by Dmitry Osin &lt;d@osin.pro&gt;</footer>
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
  .header-right { display: flex; align-items: center; }
  .btn-back { background: transparent; border: none; color: #666; font-size: 11px; font-weight: 700; letter-spacing: 1px; cursor: pointer; padding: 2px 0; transition: color 0.15s; }
  .btn-back:hover { color: #e63946; }
  .content { flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 12px; }
  .tile { background: #222; border: 1px solid #2e2e2e; padding: 14px 16px; }
  .tile-label { font-size: 10px; font-weight: 700; letter-spacing: 2px; color: #555; margin-bottom: 10px; }
  .toggle-row { display: flex; align-items: center; justify-content: space-between; padding: 6px 0; cursor: pointer; }
  .toggle-label { font-size: 12px; color: #aaa; }
  .toggle-pill { position: relative; width: 40px; height: 22px; background: #3a3a3a; border: 1px solid #4a4a4a; border-radius: 0; cursor: pointer; transition: background 0.2s; flex-shrink: 0; pointer-events: none; }
  .toggle-pill-on { background: #e63946 !important; border-color: #e63946 !important; }
  .toggle-thumb { position: absolute; top: 3px; left: 3px; width: 14px; height: 14px; background: #888; transition: transform 0.2s, background 0.2s; }
  .toggle-pill-on .toggle-thumb { transform: translateX(18px); background: #fff; }
  .save-row { display: flex; align-items: center; gap: 12px; }
  .btn-save { padding: 9px 28px; background: #e63946; border: none; color: #fff; font-size: 11px; font-weight: 700; letter-spacing: 2px; cursor: pointer; transition: background 0.15s; }
  .btn-save:hover { background: #c1121f; }
  .save-status { font-size: 11px; color: #4caf50; letter-spacing: 1px; }
  .save-error { color: #e63946; }
  .app-footer { flex-shrink: 0; padding: 8px 16px; font-size: 10px; color: #3a3a3a; border-top: 1px solid #222; text-align: right; }
</style>
