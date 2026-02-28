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

<div
    class="flex items-center justify-between h-7 px-3 bg-zinc-900/80 backdrop-blur-xl border-t border-zinc-700/50 text-xs text-zinc-400 shrink-0 select-none"
>
    <div class="flex items-center">
        {#if itemCount > 0}
            <span class="tabular-nums"
                >{itemCount} {itemCount === 1 ? "item" : "items"}</span
            >
        {/if}
    </div>

    <div class="flex items-center gap-1.5">
        <svg
            class="w-3 h-3 text-zinc-500 shrink-0"
            viewBox="0 0 16 16"
            fill="currentColor"
        >
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
            class="appearance-none w-20 h-1 bg-zinc-700 rounded-sm outline-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-[14px] [&::-webkit-slider-thumb]:h-[14px] [&::-webkit-slider-thumb]:bg-zinc-300 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:cursor-pointer [&::-webkit-slider-thumb]:shadow-[0_0.5px_2px_rgba(0,0,0,0.4)] [&::-webkit-slider-thumb]:transition-colors [&::-webkit-slider-thumb]:duration-150 hover:[&::-webkit-slider-thumb]:bg-white"
            title="Thumbnail size: {thumbnailSize}px"
        />

        <svg
            class="w-4 h-4 text-zinc-500 shrink-0"
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
