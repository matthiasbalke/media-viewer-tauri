<script lang="ts">
  import "../app.css";
  import FolderTree from "$lib/components/FolderTree.svelte";
  import MediaGrid from "$lib/components/MediaGrid.svelte";
  import MediaViewer from "$lib/components/MediaViewer.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { load } from "@tauri-apps/plugin-store";
  import { onMount } from "svelte";

  let { children } = $props();

  // normalize path for cross platform compatibility
  function normalizePath(path: string): string {
    return path.replace(/\\/g, "/");
  }

  // Array of root paths for multiple folder trees
  let rootPaths: string[] = $state([]);

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

  // Thumbnail size state
  const DEFAULT_THUMBNAIL_SIZE = 128;
  let thumbnailSize: number = $state(DEFAULT_THUMBNAIL_SIZE);
  let storeReady = $state(false);

  const storeOptions = {
    defaults: {
      thumbnailSize: DEFAULT_THUMBNAIL_SIZE,
      rootPaths: [] as string[],
    },
    autoSave: true as const,
    overrideDefaults: true,
  };

  onMount(async () => {
    const store = await load("settings.json", storeOptions);
    const savedSize = await store.get<number>("thumbnailSize");
    if (savedSize !== null && savedSize !== undefined) {
      thumbnailSize = savedSize;
    }
    const savedPaths = await store.get<string[]>("rootPaths");
    if (savedPaths && savedPaths.length > 0) {
      rootPaths = savedPaths.map(normalizePath);
    }
    storeReady = true;

    // Background cleanup of orphan thumbnails
    invoke("cleanup_orphan_thumbnails").catch((e: unknown) =>
      console.error("Orphan thumbnail cleanup failed:", e),
    );
  });

  $effect(() => {
    if (!storeReady) return;
    const size = thumbnailSize; // read synchronously so $effect tracks it
    load("settings.json", storeOptions).then((store) => {
      store.set("thumbnailSize", size);
    });
  });

  $effect(() => {
    if (!storeReady) return;
    const paths = rootPaths; // read synchronously so $effect tracks it
    load("settings.json", storeOptions).then((store) => {
      store.set("rootPaths", paths);
    });
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
      rootPaths = [...rootPaths, normalized];
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
    rootPaths = rootPaths.filter((p) => p !== pathToRemove);

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
          {thumbnailSize}
          bind:itemCount={mediaItemCount}
          bind:mediaFiles
          onImageOpen={handleImageOpen}
        />
      {/if}
    </div>
    <StatusBar bind:thumbnailSize itemCount={mediaItemCount} />
    {@render children()}
  </main>
</div>
