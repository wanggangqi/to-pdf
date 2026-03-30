import { useSettingsStore } from "@/stores/settings";
import { ElMessageBox } from "element-plus";
import { useRouter } from "vue-router";

export function useApiKeyCheck() {
  const settingsStore = useSettingsStore();
  const router = useRouter();

  async function checkApiKey(): Promise<boolean> {
    if (!settingsStore.initialized) {
      await settingsStore.loadProviders();
    }

    if (!settingsStore.hasActiveProvider) {
      ElMessageBox.alert(
        "请先在设置页面配置 AI 模型提供商的密钥，才能使用此功能。",
        "需要配置密钥",
        {
          confirmButtonText: "去设置",
          type: "warning",
        }
      ).then(() => {
        router.push("/settings");
      }).catch(() => {
        // 用户点击关闭，忽略
      });
      return false;
    }

    return true;
  }

  return {
    checkApiKey,
  };
}