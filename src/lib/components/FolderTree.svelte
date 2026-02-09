<script lang="ts">
    import FolderTree from "./FolderTree.svelte";
    import { readDir, stat } from "@tauri-apps/plugin-fs";

    interface Props {
        path: string;
        depth?: number;
    }

    let { path, depth = 0 }: Props = $props();

    // Media file extensions
    const IMAGE_EXTENSIONS = [
        "jpg",
        "jpeg",
        "png",
        "gif",
        "webp",
        "bmp",
        "svg",
        "ico",
        "avif",
    ];
    const VIDEO_EXTENSIONS = [
        "mp4",
        "webm",
        "mkv",
        "avi",
        "mov",
        "wmv",
        "flv",
        "m4v",
    ];
    const MEDIA_EXTENSIONS = [...IMAGE_EXTENSIONS, ...VIDEO_EXTENSIONS];

    interface FileEntry {
        name: string;
        path: string;
        isDirectory: boolean;
    }

    let entries: FileEntry[] = $state([]);
    let expandedDirs: Set<string> = $state(new Set());
    let loading = $state(true);
    let error: string | null = $state(null);

    function getExtension(filename: string): string {
        const parts = filename.split(".");
        return parts.length > 1 ? parts[parts.length - 1].toLowerCase() : "";
    }

    function isMediaFile(filename: string): boolean {
        return MEDIA_EXTENSIONS.includes(getExtension(filename));
    }

    function isImageFile(filename: string): boolean {
        return IMAGE_EXTENSIONS.includes(getExtension(filename));
    }

    async function loadEntries() {
        try {
            loading = true;
            error = null;

            const dirEntries = await readDir(path);
            const processed: FileEntry[] = [];

            for (const entry of dirEntries) {
                const fullPath = `${path}/${entry.name}`;
                try {
                    const info = await stat(fullPath);
                    const isDir = info.isDirectory;

                    // Show directories and media files only
                    if (isDir || isMediaFile(entry.name)) {
                        processed.push({
                            name: entry.name,
                            path: fullPath,
                            isDirectory: isDir,
                        });
                    }
                } catch {
                    // Skip files we can't stat
                }
            }

            // Sort: directories first, then by name
            entries = processed.sort((a, b) => {
                if (a.isDirectory && !b.isDirectory) return -1;
                if (!a.isDirectory && b.isDirectory) return 1;
                return a.name.localeCompare(b.name);
            });
        } catch (e) {
            error = e instanceof Error ? e.message : "Failed to read directory";
        } finally {
            loading = false;
        }
    }

    function toggleDir(dirPath: string) {
        if (expandedDirs.has(dirPath)) {
            expandedDirs.delete(dirPath);
            expandedDirs = new Set(expandedDirs);
        } else {
            expandedDirs.add(dirPath);
            expandedDirs = new Set(expandedDirs);
        }
    }

    // Load entries on mount
    $effect(() => {
        loadEntries();
    });
</script>

<div class="folder-tree" style="padding-left: {depth * 12}px">
    {#if loading && depth === 0}
        <div class="text-zinc-500 text-sm py-2">Loading...</div>
    {:else if error}
        <div class="text-red-400 text-sm py-2">{error}</div>
    {:else}
        {#each entries as entry}
            {#if entry.isDirectory}
                <button
                    class="flex items-center gap-2 w-full px-2 py-1 text-left text-sm text-zinc-300 hover:bg-zinc-800 rounded transition-colors"
                    onclick={() => toggleDir(entry.path)}
                >
                    <span class="text-zinc-500 w-4 text-center">
                        {expandedDirs.has(entry.path) ? "▼" : "▶"}
                    </span>
                    <span class="text-amber-400">📁</span>
                    <span class="truncate">{entry.name}</span>
                </button>

                {#if expandedDirs.has(entry.path)}
                    <FolderTree path={entry.path} depth={depth + 1} />
                {/if}
            {:else}
                <button
                    class="flex items-center gap-2 w-full px-2 py-1 text-left text-sm text-zinc-400 hover:bg-zinc-800 rounded transition-colors"
                >
                    <span class="w-4"></span>
                    <span>{isImageFile(entry.name) ? "🖼️" : "🎬"}</span>
                    <span class="truncate">{entry.name}</span>
                </button>
            {/if}
        {/each}

        {#if entries.length === 0 && depth === 0}
            <div class="text-zinc-500 text-sm py-2">No media files found</div>
        {/if}
    {/if}
</div>
