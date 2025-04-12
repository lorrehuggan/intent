import type { UserSettings } from "@/types/bindings";

const defaultUserSettings: UserSettings = {
  id: 1,
  defaultTimeline: "year",
  highlightCurrentDay: true,
  showCategoryFilter: false,
  theme: "light",
};

let userSettings = $state<UserSettings>(defaultUserSettings);

export function getUserSettings() {
  return userSettings;
}

export function setUserSettings(settings: UserSettings) {
  userSettings = settings;
}
