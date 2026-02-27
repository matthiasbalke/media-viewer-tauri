<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/core";

    interface MediaFile {
        name: string;
        path: string;
        isVideo: boolean;
        thumbnailState: "loading" | "ready" | "error" | "unsupported";
        thumbnailSrc: string | null;
    }

    interface Props {
        files: MediaFile[];
        selectedPath: string | null;
        onSelect?: (file: MediaFile) => void;
    }

    let { files, selectedPath, onSelect }: Props = $props();

    let stripContainer: HTMLDivElement | undefined = $state();

    // Auto-scroll to keep the selected thumbnail visible
    $effect(() => {
        if (!selectedPath || !stripContainer) return;
        const active = stripContainer.querySelector('[data-active="true"]');
        if (active) {
            active.scrollIntoView({
                behavior: "smooth",
                block: "nearest",
                inline: "center",
            });
        }
    });
</script>

<div class="filmstrip" bind:this={stripContainer}>
    {#each files as file}
        <button
            class="filmstrip-thumb"
            class:active={file.path === selectedPath}
            class:loaded={file.thumbnailState === "ready"}
            data-active={file.path === selectedPath}
            onclick={() => onSelect?.(file)}
            title={file.name}
        >
            {#if file.thumbnailState === "loading"}
                <div class="thumb-placeholder">
                    <div class="spinner"></div>
                </div>
            {:else if file.thumbnailState === "error"}
                <div class="thumb-placeholder">
                    <span class="thumb-icon error">✕</span>
                </div>
            {:else if file.thumbnailState === "unsupported"}
                <div class="thumb-placeholder">
                    <span class="thumb-icon unsupported">?</span>
                </div>
            {:else if file.thumbnailSrc}
                <img
                    src={file.thumbnailSrc}
                    alt={file.name}
                    class="thumb-img"
                    draggable="false"
                />
            {/if}
        </button>
    {/each}
</div>

<style>
    .filmstrip {
        display: flex;
        flex-direction: row;
        gap: 4px;
        padding: 6px 8px;
        overflow-x: auto;
        overflow-y: hidden;
        background: rgba(24, 24, 27, 0.9);
        border-top: 1px solid rgba(63, 63, 70, 0.5);
        flex-shrink: 0;
        scrollbar-width: thin;
        scrollbar-color: #3f3f46 transparent;
    }

    .filmstrip::-webkit-scrollbar {
        height: 4px;
    }

    .filmstrip::-webkit-scrollbar-track {
        background: transparent;
    }

    .filmstrip::-webkit-scrollbar-thumb {
        background: #3f3f46;
        border-radius: 2px;
    }

    .filmstrip-thumb {
        flex-shrink: 0;
        width: 80px;
        height: 60px;
        border: 2px solid transparent;
        border-radius: 6px;
        overflow: hidden;
        cursor: pointer;
        background: #27272a;
        padding: 0;
        transition:
            border-color 0.15s ease,
            opacity 0.15s ease;
    }

    .filmstrip-thumb:hover {
        border-color: rgba(59, 130, 246, 0.5);
    }

    .filmstrip-thumb.loaded {
        background: transparent;
    }

    .filmstrip-thumb.active {
        border-color: #3b82f6;
        box-shadow: 0 0 8px rgba(59, 130, 246, 0.3);
    }

    .thumb-img {
        width: 100%;
        height: 100%;
        object-fit: scale-down;
        display: block;
    }

    .thumb-placeholder {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        background: #18181b;
    }

    .thumb-icon {
        font-size: 18px;
    }

    .thumb-icon.error {
        color: #f87171;
    }

    .thumb-icon.unsupported {
        color: #71717a;
    }

    .spinner {
        width: 16px;
        height: 16px;
        border: 2px solid #3f3f46;
        border-top-color: #a1a1aa;
        border-radius: 50%;
        animation: spin 0.6s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
</style>
