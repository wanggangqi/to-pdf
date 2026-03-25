import { defineStore } from "pinia";
import { ref } from "vue";

export interface ProviderConfig {
  id: string;
  name: string;
  apiKey: string;
  baseUrl: string;
  model: string;
  enabled: boolean;
}

export const useSettingsStore = defineStore("settings", () => {
  const providers = ref<ProviderConfig[]>([]);
  const loading = ref(false);

  async function loadProviders() {
    // TODO: Implement loading from Tauri backend
    loading.value = true;
    try {
      // Placeholder for Tauri invoke
    } finally {
      loading.value = false;
    }
  }

  async function saveProvider(provider: ProviderConfig) {
    // TODO: Implement saving to Tauri backend
    console.log("Saving provider:", provider);
  }

  async function testProvider(providerId: string) {
    // TODO: Implement testing connection
    console.log("Testing provider:", providerId);
    return true;
  }

  return {
    providers,
    loading,
    loadProviders,
    saveProvider,
    testProvider,
  };
});