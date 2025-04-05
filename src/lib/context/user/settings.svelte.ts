import type { UserSettings } from "@/types/bindings";

let userSettings = $state<UserSettings | null>(null);

export function getUserSettings() {
  return userSettings;
}

export function setUserSettings(settings: UserSettings) {
  userSettings = settings;
}
