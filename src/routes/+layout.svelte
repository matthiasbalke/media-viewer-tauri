<script>
  import "../app.css";
  import FolderTree from "$lib/components/FolderTree.svelte";

  let { children } = $props();

  // Default to user's home directory for demo
  let rootPath = $state("/Users/matthias/Desktop/media-viewer-example");
</script>

<div class="flex h-screen bg-zinc-950 text-zinc-100">
  <!-- Sidebar -->
  <aside class="w-64 bg-zinc-900 border-r border-zinc-800 flex flex-col">
    <div class="p-4 border-b border-zinc-800">
      <h1 class="text-lg font-semibold text-white">Media Viewer</h1>
    </div>

    <nav class="flex-1 p-2 overflow-y-auto">
      <FolderTree path={rootPath} />
    </nav>
  </aside>

  <!-- Main content area -->
  <main class="flex-1 overflow-auto">
    {@render children()}
  </main>
</div>
