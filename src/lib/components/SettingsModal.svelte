<script lang="ts">
  import { settingsStore } from "$lib/stores/settings.svelte";
  import { fade, scale } from "svelte/transition";

  let { onClose } = $props<{ onClose: () => void }>();

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- Backdrop -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
  onclick={handleBackdropClick}
  transition:fade={{ duration: 200 }}
  role="dialog"
  aria-modal="true"
>
  <!-- Modal Content -->
  <div
    class="w-full max-w-lg overflow-hidden rounded-2xl bg-zinc-900 border border-zinc-800 shadow-2xl"
    transition:scale={{ duration: 200, start: 0.95 }}
  >
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-zinc-800 px-6 py-4">
      <h2 class="text-xl font-semibold text-white">Settings</h2>
      <button
        class="flex h-8 w-8 items-center justify-center rounded-full text-zinc-400 hover:bg-zinc-800 hover:text-white transition-colors"
        onclick={onClose}
        aria-label="Close"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>

    <!-- Body -->
    <div class="p-6">
      <!-- Section: Display -->
      <div class="space-y-6">
        <div>
          <h3 class="text-sm font-medium text-zinc-400 uppercase tracking-wider mb-4">Display</h3>
          
          <div class="space-y-2">
            <div class="flex justify-between items-center">
              <label for="thumbnailSize" class="text-sm font-medium text-zinc-200">
                Default Thumbnail Size
              </label>
              <span class="text-xs text-zinc-500 font-mono bg-zinc-950 px-2 py-1 rounded">
                {settingsStore.thumbnailSize}px
              </span>
            </div>
            <input
              id="thumbnailSize"
              type="range"
              min="64"
              max="512"
              step="32"
              value={settingsStore.thumbnailSize}
              oninput={(e) => settingsStore.saveSize(Number(e.currentTarget.value))}
              class="w-full h-2 bg-zinc-800 rounded-lg appearance-none cursor-pointer accent-blue-500"
            />
            <p class="text-xs text-zinc-500 mt-1">
              Adjusts the default size of media grid items.
            </p>
          </div>
        </div>

        <!-- Section: Application -->
        <div class="border-t border-zinc-800/50 pt-6">
          <h3 class="text-sm font-medium text-zinc-400 uppercase tracking-wider mb-4">Storage</h3>
          
          <div class="space-y-4">
            <div>
              <p class="text-sm font-medium text-zinc-200">Tracked Directories</p>
              <p class="text-xs text-zinc-500 mt-1 mb-2">
                Locations actively scanned for media. Use the sidebar to add new directories.
              </p>
              
              {#if settingsStore.rootPaths.length === 0}
                <div class="text-xs text-zinc-600 italic bg-zinc-950/50 rounded p-2 text-center border border-zinc-800/50">
                  No directories added yet.
                </div>
              {:else}
                <ul class="space-y-1 max-h-40 overflow-y-auto pr-2 custom-scrollbar">
                  {#each settingsStore.rootPaths as path}
                    <li class="text-xs text-zinc-300 bg-zinc-800/50 px-3 py-2 rounded font-mono truncate border border-zinc-800">
                      {path}
                    </li>
                  {/each}
                </ul>
              {/if}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  /* Optional custom scrollbar for the paths list */
  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background-color: #3f3f46;
    border-radius: 20px;
  }
</style>
