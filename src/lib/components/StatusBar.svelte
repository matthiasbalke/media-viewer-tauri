<script lang="ts">
    interface Props {
        thumbnailSize: number;
        steps?: number[];
        itemCount?: number;
    }

    let {
        thumbnailSize = $bindable(128),
        steps = [64, 128, 256, 512],
        itemCount = 0,
    }: Props = $props();

    // Map slider index (0..steps.length-1) to actual step value
    let sliderIndex = $derived(steps.indexOf(thumbnailSize));

    function handleInput(e: Event) {
        const target = e.target as HTMLInputElement;
        const index = parseInt(target.value, 10);
        thumbnailSize = steps[index];
    }
</script>

<div class="statusbar">
    <div class="statusbar-left">
        {#if itemCount > 0}
            <span class="item-count"
                >{itemCount} {itemCount === 1 ? "item" : "items"}</span
            >
        {/if}
    </div>

    <div class="statusbar-right">
        <svg class="size-icon" viewBox="0 0 16 16" fill="currentColor">
            <rect x="3" y="3" width="4" height="4" rx="0.5" />
            <rect x="9" y="3" width="4" height="4" rx="0.5" />
            <rect x="3" y="9" width="4" height="4" rx="0.5" />
            <rect x="9" y="9" width="4" height="4" rx="0.5" />
        </svg>

        <input
            type="range"
            min="0"
            max={steps.length - 1}
            step="1"
            value={sliderIndex}
            oninput={handleInput}
            class="size-slider"
            title="Thumbnail size: {thumbnailSize}px"
        />

        <svg
            class="size-icon size-icon-large"
            viewBox="0 0 16 16"
            fill="currentColor"
        >
            <rect x="1" y="1" width="6" height="6" rx="0.5" />
            <rect x="9" y="1" width="6" height="6" rx="0.5" />
            <rect x="1" y="9" width="6" height="6" rx="0.5" />
            <rect x="9" y="9" width="6" height="6" rx="0.5" />
        </svg>
    </div>
</div>

<style>
    .statusbar {
        display: flex;
        align-items: center;
        justify-content: space-between;
        height: 28px;
        padding: 0 12px;
        background: rgba(24, 24, 27, 0.8);
        backdrop-filter: blur(12px);
        -webkit-backdrop-filter: blur(12px);
        border-top: 1px solid rgba(63, 63, 70, 0.5);
        font-size: 12px;
        color: #a1a1aa;
        flex-shrink: 0;
        user-select: none;
    }

    .statusbar-left {
        display: flex;
        align-items: center;
    }

    .item-count {
        font-variant-numeric: tabular-nums;
    }

    .statusbar-right {
        display: flex;
        align-items: center;
        gap: 6px;
    }

    .size-icon {
        width: 12px;
        height: 12px;
        color: #71717a;
        flex-shrink: 0;
    }

    .size-icon-large {
        width: 16px;
        height: 16px;
    }

    /* Range slider — macOS-inspired minimal style */
    .size-slider {
        -webkit-appearance: none;
        appearance: none;
        width: 80px;
        height: 4px;
        background: #3f3f46;
        border-radius: 2px;
        outline: none;
        cursor: pointer;
    }

    .size-slider::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 14px;
        height: 14px;
        background: #d4d4d8;
        border-radius: 50%;
        cursor: pointer;
        box-shadow: 0 0.5px 2px rgba(0, 0, 0, 0.4);
        transition: background 0.15s ease;
    }

    .size-slider::-webkit-slider-thumb:hover {
        background: #ffffff;
    }

    .size-slider::-moz-range-thumb {
        width: 14px;
        height: 14px;
        background: #d4d4d8;
        border: none;
        border-radius: 50%;
        cursor: pointer;
        box-shadow: 0 0.5px 2px rgba(0, 0, 0, 0.4);
        transition: background 0.15s ease;
    }

    .size-slider::-moz-range-thumb:hover {
        background: #ffffff;
    }
</style>
