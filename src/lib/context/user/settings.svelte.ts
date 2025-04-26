import type { UserSettings } from "@/types/bindings";

let settings = $state<UserSettings | null>(null);

export const userSettings = {
  get value() {
    if (!settings)
      return {
        defaultTimeline: "year",
        highlightCurrentDay: true,
        id: 1,
        showCategoryFilter: false,
        theme: "dark",
      };
    return settings;
  },
  set value(updatedSettings: UserSettings) {
    settings = updatedSettings;
  },
};
