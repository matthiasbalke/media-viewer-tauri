<script lang="ts">
  import "../app.css";
  import FolderTree from "$lib/components/FolderTree.svelte";
  import MediaGrid from "$lib/components/MediaGrid.svelte";
  import MediaViewer from "$lib/components/MediaViewer.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { settingsStore } from "$lib/stores/settings.svelte";
  import SettingsModal from "$lib/components/SettingsModal.svelte";

  let { children } = $props();

  // normalize path for cross platform compatibility
  function normalizePath(path: string): string {
    return path.replace(/\\/g, "/");
  }

  // Currently selected directory for media grid
  let selectedPath: string | null = $state(null);

  // Currently viewed file (null = grid mode, set = viewer mode)
  interface MediaFile {
    name: string;
    path: string;
    isVideo: boolean;
    thumbnailState: "loading" | "ready" | "error" | "unsupported";
    thumbnailSrc: string | null;
  }
  let viewingFile: MediaFile | null = $state(null);
  let mediaFiles: MediaFile[] = $state([]);
  let showSettings = $state(false);

  onMount(async () => {
    // Background cleanup of orphan thumbnails
    invoke("cleanup_orphan_thumbnails").catch((e: unknown) =>
      console.error("Orphan thumbnail cleanup failed:", e),
    );
  });

  // Item count from media grid
  let mediaItemCount: number = $state(0);

  function handleFolderSelect(path: string) {
    selectedPath = path;
    viewingFile = null; // reset viewer when switching folders
  }

  function handleImageOpen(file: MediaFile) {
    viewingFile = file;
  }

  function handleViewerClose() {
    viewingFile = null;
  }

  function handleViewerFileChange(file: MediaFile) {
    viewingFile = file;
  }

  async function addFolderTree() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select Root Directory",
    });

    if (selected && typeof selected === "string") {
      const normalized = normalizePath(selected);
      settingsStore.addRootPath(normalized);
      // Auto-select the new directory
      selectedPath = normalized;
    }
  }

  async function removeRootPath(pathToRemove: string) {
    // Clean up thumbnails for this directory tree
    try {
      await invoke("cleanup_thumbnails_for_dir", { dir: pathToRemove });
    } catch (e) {
      console.error("Thumbnail cleanup failed:", e);
    }

    // Remove from rootPaths
    settingsStore.removeRootPath(pathToRemove);

    // Clear selection if it was within the removed tree
    if (selectedPath?.startsWith(pathToRemove)) {
      selectedPath = null;
    }
  }
</script>

<div class="flex h-screen bg-zinc-950 text-zinc-100">
  <!-- Sidebar -->
  <aside class="w-64 bg-zinc-900 border-r border-zinc-800 flex flex-col">
    <div class="p-4 border-b border-zinc-800 flex items-center justify-between">
      <h1 class="text-lg font-semibold text-white">Media Viewer</h1>
      <div class="flex items-center gap-2">
        <button
          class="w-8 h-8 flex items-center justify-center rounded-lg bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-white transition-colors"
          onclick={() => (showSettings = true)}
          title="Settings"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <circle cx="12" cy="12" r="3"></circle>
            <path
              d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
            ></path>
          </svg>
        </button>
        <button
          class="w-8 h-8 flex items-center justify-center rounded-lg bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-white transition-colors"
          onclick={addFolderTree}
          title="Add folder"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            ><line x1="12" y1="5" x2="12" y2="19"></line><line
              x1="5"
              y1="12"
              x2="19"
              y2="12"
            ></line></svg
          >
        </button>
      </div>
    </div>

    <nav class="flex-1 p-2 overflow-y-auto">
      {#if settingsStore.rootPaths.length === 0}
        <div class="text-zinc-500 text-sm py-4 text-center">
          Click the + icon to add a folder
        </div>
      {:else}
        {#each settingsStore.rootPaths as rootPath}
          <div class="mb-2 group/root relative">
            <FolderTree path={rootPath} onSelect={handleFolderSelect} />
            <button
              class="absolute top-0 right-0 w-6 h-6 flex items-center justify-center
                     text-zinc-500 hover:text-red-400 rounded
                     opacity-0 group-hover/root:opacity-100 transition-opacity
                     text-xs"
              onclick={() => removeRootPath(rootPath)}
              title="Remove directory"
            >
              ✕
            </button>
          </div>
        {/each}
      {/if}
    </nav>
  </aside>

  <!-- Main content area -->
  <main class="flex-1 flex flex-col overflow-hidden">
    <div class="flex-1 overflow-auto">
      {#if viewingFile}
        <MediaViewer
          file={viewingFile}
          files={mediaFiles}
          onClose={handleViewerClose}
          onFileChange={handleViewerFileChange}
        />
      {:else}
        <MediaGrid
          path={selectedPath}
          thumbnailSize={settingsStore.thumbnailSize}
          bind:itemCount={mediaItemCount}
          bind:mediaFiles
          onImageOpen={handleImageOpen}
        />
      {/if}
    </div>
    <StatusBar
      bind:thumbnailSize={settingsStore.thumbnailSize}
      itemCount={mediaItemCount}
    />
    {@render children()}
  </main>
</div>

{#if showSettings}
  <SettingsModal onClose={() => (showSettings = false)} />
{/if}
