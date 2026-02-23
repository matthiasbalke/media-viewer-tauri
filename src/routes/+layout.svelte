<script lang="ts">
  import "../app.css";
  import FolderTree from "$lib/components/FolderTree.svelte";
  import MediaGrid from "$lib/components/MediaGrid.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import { open } from "@tauri-apps/plugin-dialog";

  let { children } = $props();

  // Array of root paths for multiple folder trees
  let rootPaths: string[] = $state([]);

  // Currently selected directory for media grid
  let selectedPath: string | null = $state(null);

  // Thumbnail size state
  let thumbnailSize: number = $state(128);

  // Item count from media grid
  let mediaItemCount: number = $state(0);

  function handleFolderSelect(path: string) {
    selectedPath = path;
  }

  async function addFolderTree() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select Root Directory",
    });

    if (selected && typeof selected === "string") {
      rootPaths = [...rootPaths, selected];
      // Auto-select the new directory
      selectedPath = selected;
    }
  }
</script>

<div class="flex h-screen bg-zinc-950 text-zinc-100">
  <!-- Sidebar -->
  <aside class="w-64 bg-zinc-900 border-r border-zinc-800 flex flex-col">
    <div class="p-4 border-b border-zinc-800 flex items-center justify-between">
      <h1 class="text-lg font-semibold text-white">Media Viewer</h1>
      <button
        class="w-8 h-8 flex items-center justify-center rounded-lg bg-zinc-800 hover:bg-zinc-700 text-zinc-400 hover:text-white transition-colors"
        onclick={addFolderTree}
        title="Add folder"
      >
        +
      </button>
    </div>

    <nav class="flex-1 p-2 overflow-y-auto">
      {#if rootPaths.length === 0}
        <div class="text-zinc-500 text-sm py-4 text-center">
          Click + to add a folder
        </div>
      {:else}
        {#each rootPaths as rootPath}
          <div class="mb-2">
            <FolderTree path={rootPath} onSelect={handleFolderSelect} />
          </div>
        {/each}
      {/if}
    </nav>
  </aside>

  <!-- Main content area -->
  <main class="flex-1 flex flex-col overflow-hidden">
    <div class="flex-1 overflow-auto">
      <MediaGrid path={selectedPath} thumbnailSize={thumbnailSize} bind:itemCount={mediaItemCount} />
    </div>
    <StatusBar bind:thumbnailSize={thumbnailSize} itemCount={mediaItemCount} />
    {@render children()}
  </main>
</div>
