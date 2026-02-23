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

<div class="viewer-container">
    <div class="viewer-main">
        <!-- Close button -->
        <button
            class="close-btn"
            onclick={() => onClose?.()}
            title="Back to grid (Esc)"
        >
            <svg
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
            >
                <path d="M19 12H5M12 19l-7-7 7-7" />
            </svg>
        </button>

        {#if imageSrc}
            <img src={imageSrc} alt={file.name} class="viewer-image" />
        {:else}
            <div class="unsupported-preview">
                <span class="unsupported-icon">🖼️</span>
                <p class="unsupported-text">
                    Preview not available for .{getExtension(file.name)} files
                </p>
            </div>
        {/if}

        <!-- File name -->
        <div class="file-name">{file.name}</div>
    </div>

    <Filmstrip
        {files}
        selectedPath={file.path}
        onSelect={handleFilmstripSelect}
    />
</div>

<style>
    .viewer-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        background: #09090b;
    }

    .viewer-main {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;
        overflow: hidden;
        min-height: 0;
    }

    .close-btn {
        position: absolute;
        top: 12px;
        left: 12px;
        z-index: 10;
        width: 36px;
        height: 36px;
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(24, 24, 27, 0.7);
        backdrop-filter: blur(8px);
        -webkit-backdrop-filter: blur(8px);
        border: 1px solid rgba(63, 63, 70, 0.5);
        border-radius: 8px;
        color: #a1a1aa;
        cursor: pointer;
        padding: 0;
        transition:
            color 0.15s ease,
            background 0.15s ease;
    }

    .close-btn:hover {
        color: #ffffff;
        background: rgba(39, 39, 42, 0.9);
    }

    .close-btn svg {
        width: 18px;
        height: 18px;
    }

    .viewer-image {
        max-width: 100%;
        max-height: 100%;
        object-fit: contain;
        border-radius: 4px;
    }

    .unsupported-preview {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 12px;
        color: #71717a;
    }

    .unsupported-icon {
        font-size: 48px;
    }

    .unsupported-text {
        font-size: 14px;
        margin: 0;
    }

    .file-name {
        position: absolute;
        bottom: 12px;
        left: 50%;
        transform: translateX(-50%);
        padding: 4px 12px;
        background: rgba(24, 24, 27, 0.7);
        backdrop-filter: blur(8px);
        -webkit-backdrop-filter: blur(8px);
        border-radius: 6px;
        font-size: 12px;
        color: #a1a1aa;
        white-space: nowrap;
        max-width: 80%;
        overflow: hidden;
        text-overflow: ellipsis;
    }
</style>
