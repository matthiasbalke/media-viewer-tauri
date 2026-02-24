import { load } from "@tauri-apps/plugin-store";

const DEFAULT_THUMBNAIL_SIZE = 128;
const STORE_NAME = "settings.json";

const storeOptions = {
    defaults: {
        thumbnailSize: DEFAULT_THUMBNAIL_SIZE,
        rootPaths: [] as string[],
    },
    autoSave: true as const,
    overrideDefaults: true,
};

class SettingsStore {
    thumbnailSize = $state(DEFAULT_THUMBNAIL_SIZE);
    rootPaths = $state<string[]>([]);
    ready = $state(false);

    private store: any = null;
    private saveTimeout: ReturnType<typeof setTimeout> | null = null;

    constructor() {
        this.init();
    }

    async init() {
        try {
            this.store = await load(STORE_NAME, storeOptions);

            const savedSize = await this.store.get("thumbnailSize") as number | null | undefined;
            if (savedSize !== null && savedSize !== undefined) {
                this.thumbnailSize = savedSize;
            }

            const savedPaths = await this.store.get("rootPaths") as string[] | null | undefined;
            if (savedPaths && savedPaths.length > 0) {
                this.rootPaths = savedPaths;
            }
        } catch (error) {
            console.error("Failed to load settings:", error);
        } finally {
            this.ready = true;
        }
    }

    // Debounced save for rapidly changing values like slider
    async saveSize(size: number) {
        this.thumbnailSize = size;
        this.debouncedSave("thumbnailSize", size);
    }

    async addRootPath(path: string) {
        if (!this.rootPaths.includes(path)) {
            this.rootPaths = [...this.rootPaths, path];
            await this.saveNow("rootPaths", this.rootPaths);
        }
    }

    async removeRootPath(path: string) {
        this.rootPaths = this.rootPaths.filter(p => p !== path);
        await this.saveNow("rootPaths", this.rootPaths);
    }

    private debouncedSave(key: string, value: any) {
        if (!this.ready || !this.store) return;

        if (this.saveTimeout) {
            clearTimeout(this.saveTimeout);
        }

        this.saveTimeout = setTimeout(async () => {
            try {
                await this.store.set(key, value);
            } catch (error) {
                console.error(`Failed to save ${key}:`, error);
            }
        }, 500); // 500ms debounce
    }

    private async saveNow(key: string, value: any) {
        if (!this.ready || !this.store) return;
        try {
            await this.store.set(key, value);
        } catch (error) {
            console.error(`Failed to save ${key}:`, error);
        }
    }
}

export const settingsStore = new SettingsStore();
