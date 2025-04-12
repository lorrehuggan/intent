<script lang="ts">
  import Switch from "@/components/ui/switch/switch.svelte";
  import { getUserSettings, setUserSettings } from "@/context/user/settings.svelte";
  import type { UserSettings } from "@/types/bindings";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { tv } from "tailwind-variants";

  const html = document.querySelector("html");

  let darkModeToggle = $state(false);
  let highlightCurrentDayToggle = $state(false);

  onMount(() => {
    const userSettings = getUserSettings();
    if (!html) return;
    if (html.classList.contains("dark")) {
      darkModeToggle = true;
    } else {
      darkModeToggle = false;
    }
    if (!userSettings) return;
    highlightCurrentDayToggle = userSettings?.highlightCurrentDay;
  });

  async function toggleDarkMode() {
    const userSettings = getUserSettings();
    if (!userSettings || !html) return;
    const classList = html.classList;
    if (classList.contains("dark")) {
      classList.remove("dark");
      darkModeToggle = false;
      const payload = {
        ...userSettings,
        theme: "light",
      };
      try {
        setUserSettings({ ...userSettings, theme: "light" });
        await invoke("set_user_settings", { payload });
      } catch (e) {
        console.error(e);
      }
    } else {
      classList.add("dark");
      darkModeToggle = true;
      const payload = {
        ...userSettings,
        theme: "dark",
      };
      try {
        setUserSettings({ ...userSettings, theme: "dark" });
        await invoke("set_user_settings", { payload });
      } catch (e) {
        console.error(e);
      }
    }
  }

  async function toggleHighlightCurrentDay() {
    const userSettings = getUserSettings();
    if (!userSettings) return;
    if (highlightCurrentDayToggle === true) {
      highlightCurrentDayToggle = false;
      const payload: UserSettings = {
        ...userSettings,
        highlightCurrentDay: false,
      };
      try {
        await invoke("set_user_settings", {
          payload,
        });
      } catch (e) {
        console.error(e);
      }
    } else if (highlightCurrentDayToggle === false) {
      highlightCurrentDayToggle = true;
      const payload: UserSettings = {
        ...userSettings,
        highlightCurrentDay: true,
      };
      try {
        await invoke("set_user_settings", {
          payload,
        });
      } catch (e) {
        console.error(e);
      }
    } else {
      return;
    }
  }

  const styles = tv({
    slots: {
      header: "page-heading",
      settingContainer:
        "border-b-muted-foreground/20 mx-auto mt-8 w-full max-w-[1200px] justify-between gap-4 border-b pb-4",
      settingList: "mt-2 space-y-2",
      settingCategory: "text-lg",
      settingTitle: "text-muted-foreground text-sm",
    },
  });

  const { header, settingTitle, settingCategory, settingContainer, settingList } = styles();
</script>

<h1 class={header()}>Settings</h1>
<!-- Appearance -->
<div class={settingContainer()}>
  <h4 class={settingCategory()}>Appearance</h4>
  <div class={settingList()}>
    <div class="flex-center justify-between">
      <p class={settingTitle()}>
        {`Set theme to ${darkModeToggle ? "light" : "dark"} mode`}
      </p>
      <Switch onCheckedChange={toggleDarkMode} bind:checked={darkModeToggle} />
    </div>
    <div class="flex-center justify-between">
      <p class={settingTitle()}>Highlight the current day</p>
      <Switch
        onCheckedChange={toggleHighlightCurrentDay}
        bind:checked={highlightCurrentDayToggle}
      />
    </div>
  </div>
</div>
