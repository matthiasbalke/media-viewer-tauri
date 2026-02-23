<script lang="ts">
    import { readDir, stat } from "@tauri-apps/plugin-fs";
    import { convertFileSrc } from "@tauri-apps/api/core";

    interface Props {
        path: string | null;
        thumbnailSize?: number;
        itemCount?: number;
    }

    let {
        path,
        thumbnailSize = 128,
        itemCount = $bindable(0),
    }: Props = $props();

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

    interface MediaFile {
        name: string;
        path: string;
        src: string;
        isVideo: boolean;
    }

    let files: MediaFile[] = $state([]);
    let loading = $state(false);
    let error: string | null = $state(null);

    function getExtension(filename: string): string {
        const parts = filename.split(".");
        return parts.length > 1 ? parts[parts.length - 1].toLowerCase() : "";
    }

    function isImageFile(filename: string): boolean {
        return IMAGE_EXTENSIONS.includes(getExtension(filename));
    }

    function isVideoFile(filename: string): boolean {
        return VIDEO_EXTENSIONS.includes(getExtension(filename));
    }

    function isMediaFile(filename: string): boolean {
        return isImageFile(filename) || isVideoFile(filename);
    }

    async function loadMedia(dirPath: string) {
        try {
            loading = true;
            error = null;
            files = [];

            const entries = await readDir(dirPath);
            const mediaFiles: MediaFile[] = [];

            for (const entry of entries) {
                if (isMediaFile(entry.name)) {
                    const fullPath = `${dirPath}/${entry.name}`;
                    try {
                        const info = await stat(fullPath);
                        if (!info.isDirectory) {
                            mediaFiles.push({
                                name: entry.name,
                                path: fullPath,
                                src: convertFileSrc(fullPath),
                                isVideo: isVideoFile(entry.name),
                            });
                        }
                    } catch {
                        // Skip files we can't stat
                    }
                }
            }

            // Sort by name
            files = mediaFiles.sort((a, b) => a.name.localeCompare(b.name));
        } catch (e) {
            error = e instanceof Error ? e.message : "Failed to load media";
        } finally {
            loading = false;
        }
    }

    // Reload when path changes
    $effect(() => {
        if (path) {
            loadMedia(path);
        } else {
            files = [];
        }
    });

    // Sync item count for statusbar
    $effect(() => {
        itemCount = files.length;
    });
</script>

<div class="p-4">
    {#if !path}
        <div class="flex items-center justify-center h-full text-zinc-500">
            <p>Select a folder to view media</p>
        </div>
    {:else if loading}
        <div class="flex items-center justify-center h-32 text-zinc-500">
            <p>Loading...</p>
        </div>
    {:else if error}
        <div class="text-red-400 p-4">{error}</div>
    {:else if files.length === 0}
        <div class="flex items-center justify-center h-32 text-zinc-500">
            <p>No media files in this folder</p>
        </div>
    {:else}
        <div
            class="grid gap-4"
            style="grid-template-columns: repeat(auto-fill, minmax({thumbnailSize}px, 1fr));"
        >
            {#each files as file}
                <div
                    class="group relative bg-zinc-800 rounded-lg overflow-hidden hover:ring-2 hover:ring-blue-500 transition-all cursor-pointer"
                    style="height: {thumbnailSize}px;"
                >
                    {#if file.isVideo}
                        <div
                            class="absolute inset-0 flex items-center justify-center bg-zinc-900"
                        >
                            <span class="text-4xl">🎬</span>
                        </div>
                        <div
                            class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-2"
                        >
                            <p class="text-xs text-zinc-300 truncate">
                                {file.name}
                            </p>
                        </div>
                    {:else}
                        <img
                            src={file.src}
                            alt={file.name}
                            class="w-full h-full object-cover"
                            loading="lazy"
                        />
                        <div
                            class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-2 opacity-0 group-hover:opacity-100 transition-opacity"
                        >
                            <p class="text-xs text-zinc-300 truncate">
                                {file.name}
                            </p>
                        </div>
                    {/if}
                </div>
            {/each}
        </div>
    {/if}
</div>
