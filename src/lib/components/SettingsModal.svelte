<script lang="ts">
    import { fade, scale } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/core";

    let { onClose } = $props<{ onClose: () => void }>();

    let isCleaning = $state(false);
    let cleanMessage = $state("");

    async function handleCleanup() {
        isCleaning = true;
        cleanMessage = "Cleaning up...";
        try {
            await invoke("cleanup_orphan_thumbnails");
            cleanMessage = "Cleanup successful!";
        } catch (e) {
            console.error(e);
            cleanMessage = "Failed to cleanup cache.";
        } finally {
            setTimeout(() => {
                isCleaning = false;
                cleanMessage = "";
            }, 3000);
        }
    }

    function handleBackdropClick(e: MouseEvent) {
        if (e.target === e.currentTarget) {
            onClose();
        }
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Escape") {
            onClose();
        }
    }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- Backdrop -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
    onclick={handleBackdropClick}
    transition:fade={{ duration: 200 }}
    role="dialog"
    aria-modal="true"
>
    <!-- Modal Content -->
    <div
        class="w-full max-w-lg overflow-hidden rounded-2xl bg-zinc-900 border border-zinc-800 shadow-2xl"
        transition:scale={{ duration: 200, start: 0.95 }}
    >
        <!-- Header -->
        <div
            class="flex items-center justify-between border-b border-zinc-800 px-6 py-4"
        >
            <h2 class="text-xl font-semibold text-white">Settings</h2>
            <button
                class="flex h-8 w-8 items-center justify-center rounded-full text-zinc-400 hover:bg-zinc-800 hover:text-white transition-colors"
                onclick={onClose}
                aria-label="Close"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
            </button>
        </div>

        <!-- Body -->
        <div class="p-6">
            <!-- Section: Cache Management -->
            <div class="space-y-6">
                <div>
                    <h3
                        class="text-sm font-medium text-zinc-400 uppercase tracking-wider mb-4"
                    >
                        Cache Management
                    </h3>

                    <div class="space-y-4">
                        <div class="flex items-start justify-between">
                            <div>
                                <p class="text-sm font-medium text-zinc-200">
                                    Orphan Thumbnails
                                </p>
                                <p class="text-xs text-zinc-500 mt-1 max-w-sm">
                                    Remove cached thumbnails for media files
                                    that no longer exist on disk. This can help
                                    free up storage space.
                                </p>
                            </div>
                            <button
                                class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-sm font-medium text-white rounded-lg transition-colors border border-zinc-700 disabled:opacity-50 disabled:cursor-not-allowed"
                                onclick={handleCleanup}
                                disabled={isCleaning}
                            >
                                {isCleaning ? "Cleaning..." : "Clean Up"}
                            </button>
                        </div>
                        {#if cleanMessage}
                            <p class="text-xs text-blue-400" transition:fade>
                                {cleanMessage}
                            </p>
                        {/if}
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
