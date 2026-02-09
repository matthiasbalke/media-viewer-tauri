<script>
  import "../app.css";
</script>

<div class="flex h-screen bg-zinc-950 text-zinc-100">
  <!-- Sidebar -->
  <aside class="w-64 bg-zinc-900 border-r border-zinc-800 flex flex-col">
    <div class="p-4 border-b border-zinc-800">
      <h1 class="text-lg font-semibold text-white">Media Viewer</h1>
    </div>

    <nav class="flex-1 p-2 overflow-y-auto">
      <!-- Navigation items will go here -->
    </nav>
  </aside>

  <!-- Main content area -->
  <main class="flex-1 overflow-auto">
    <slot />
  </main>
</div>
