<script lang="ts">
  import "../app.css";
  import FolderTree from "$lib/components/FolderTree.svelte";
  import MediaGrid from "$lib/components/MediaGrid.svelte";

  let { children } = $props();

  // Default root path for folder tree
  let rootPath = $state("/Users/matthias/Desktop/media-viewer-example");

  // Currently selected directory for media grid
  let selectedPath: string | null = $state(rootPath);

  function handleFolderSelect(path: string) {
    selectedPath = path;
  }
</script>

<div class="flex h-screen bg-zinc-950 text-zinc-100">
  <!-- Sidebar -->
  <aside class="w-64 bg-zinc-900 border-r border-zinc-800 flex flex-col">
    <div class="p-4 border-b border-zinc-800">
      <h1 class="text-lg font-semibold text-white">Media Viewer</h1>
    </div>

    <nav class="flex-1 p-2 overflow-y-auto">
      <FolderTree path={rootPath} onSelect={handleFolderSelect} />
    </nav>
  </aside>

  <!-- Main content area -->
  <main class="flex-1 overflow-auto">
    <MediaGrid path={selectedPath} />
    {@render children()}
  </main>
</div>
