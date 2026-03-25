import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ProviderConfig } from "@/types";

export const useSettingsStore = defineStore("settings", () => {
  const providers = ref<ProviderConfig[]>([]);
  const loading = ref(false);

  const activeProvider = computed(
    () => providers.value.find((p) => p.isActive) || null
  );

  async function loadProviders() {
    loading.value = true;
    try {
      const result = await invoke<ProviderConfig[]>("get_providers");
      providers.value = result;
    } catch (error) {
      console.error("Failed to load providers:", error);
      providers.value = [];
    } finally {
      loading.value = false;
    }
  }

  async function saveProvider(provider: ProviderConfig) {
    try {
      await invoke("save_provider", { provider });
      await loadProviders();
    } catch (error) {
      console.error("Failed to save provider:", error);
      throw error;
    }
  }

  async function testProvider(provider: ProviderConfig): Promise<string> {
    try {
      const result = await invoke<string>("test_provider", { provider });
      return result;
    } catch (error) {
      console.error("Failed to test provider:", error);
      throw error;
    }
  }

  return {
    providers,
    loading,
    activeProvider,
    loadProviders,
    saveProvider,
    testProvider,
  };
});