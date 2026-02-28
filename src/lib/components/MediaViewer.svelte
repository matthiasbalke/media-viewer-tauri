<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/core";
    import Filmstrip from "./Filmstrip.svelte";

    interface MediaFile {
        name: string;
        path: string;
        isVideo: boolean;
        thumbnailState: "loading" | "ready" | "error" | "unsupported";
        thumbnailSrc: string | null;
    }

    interface Props {
        file: MediaFile;
        files: MediaFile[];
        onClose?: () => void;
        onFileChange?: (file: MediaFile) => void;
    }

    let { file, files, onClose, onFileChange }: Props = $props();

    // WebKit-supported image formats that can be displayed natively
    const WEBVIEW_SUPPORTED = [
        "jpg",
        "jpeg",
        "png",
        "gif",
        "webp",
        "svg",
        "avif",
        "bmp",
        "ico",
    ];

    function getExtension(filename: string): string {
        const parts = filename.split(".");
        return parts.length > 1 ? parts[parts.length - 1].toLowerCase() : "";
    }

    function isWebViewSupported(filename: string): boolean {
        return WEBVIEW_SUPPORTED.includes(getExtension(filename));
    }

    let imageSrc = $derived(
        isWebViewSupported(file.name) ? convertFileSrc(file.path) : null,
    );

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Escape") {
            onClose?.();
            return;
        }

        // Navigation
        if (event.key === "ArrowRight" || event.key === "ArrowLeft") {
            const currentIndex = files.findIndex((f) => f.path === file.path);
            if (currentIndex === -1) return;

            if (event.key === "ArrowRight" && currentIndex < files.length - 1) {
                onFileChange?.(files[currentIndex + 1]);
            } else if (event.key === "ArrowLeft" && currentIndex > 0) {
                onFileChange?.(files[currentIndex - 1]);
            }
        }
    }

    function handleFilmstripSelect(selected: MediaFile) {
        onFileChange?.(selected);
    }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex flex-col h-full bg-zinc-950">
    <div
        class="flex-1 flex items-center justify-center relative overflow-hidden min-h-0"
    >
        <!-- Close button -->
        <button
            class="absolute top-3 left-3 z-10 w-9 h-9 flex items-center justify-center bg-zinc-900/70 backdrop-blur-md border border-zinc-700/50 rounded-lg text-zinc-400 cursor-pointer p-0 transition-colors duration-150 hover:text-white hover:bg-zinc-800/90"
            onclick={() => onClose?.()}
            title="Back to grid (Esc)"
        >
            <svg
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                class="w-[18px] h-[18px]"
            >
                <path d="M19 12H5M12 19l-7-7 7-7" />
            </svg>
        </button>

        {#if imageSrc}
            <img
                src={imageSrc}
                alt={file.name}
                class="max-w-full max-h-full object-contain rounded"
            />
        {:else}
            <div class="flex flex-col items-center gap-3 text-zinc-500">
                <span class="text-5xl">🖼️</span>
                <p class="text-sm m-0">
                    Preview not available for .{getExtension(file.name)} files
                </p>
            </div>
        {/if}

        <!-- File name -->
        <div
            class="absolute bottom-3 left-1/2 -translate-x-1/2 px-3 py-1 bg-zinc-900/70 backdrop-blur-md rounded-md text-xs text-zinc-400 whitespace-nowrap max-w-[80%] overflow-hidden text-ellipsis"
        >
            {file.name}
        </div>
    </div>

    <Filmstrip
        {files}
        selectedPath={file.path}
        onSelect={handleFilmstripSelect}
    />
</div>
