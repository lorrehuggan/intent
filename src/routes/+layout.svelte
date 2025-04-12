<script lang="ts">
  import Sidebar from "@/components/app/sidebar/sidebar.svelte";
  import Titlebar from "@/components/app/titlebar/titlebar.svelte";
  import { tv } from "tailwind-variants";
  import "../styles/app.css";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { UserSettings } from "@/types/bindings";
  import { getUserSettings, setUserSettings } from "@/context/user/settings.svelte";

  const html = document.querySelector("html");

  let { children } = $props();

  onMount(async () => {
    invoke<UserSettings>("get_user_settings").then((settings) => setUserSettings(settings));

    const userSettings: UserSettings = getUserSettings();

    if (userSettings.theme === "light") {
      html?.classList.remove("dark");
    } else if (userSettings.theme === "dark") {
      html?.classList.add("dark");
    } else {
      html?.classList.remove("dark");
    }
  });

  const styles = tv({
    slots: {
      windowContainer: "h-screen w-screen overflow-hidden rounded-2xl",
      appContainer: "flex h-[calc(100vh-36px)] overflow-hidden",
      contentContainer: "w-full overflow-x-hidden p-8 pt-2 pl-2",
      mainContainer:
        "h-full overflow-y-scroll rounded-2xl bg-neutral-100 px-6 py-4 dark:bg-neutral-900",
    },
  });

  const { windowContainer, appContainer, contentContainer, mainContainer } = styles();
</script>

<div class={windowContainer()}>
  <Titlebar />
  <div class={appContainer()}>
    <Sidebar />
    <div class={contentContainer()}>
      <main class={mainContainer()}>
        {@render children()}
      </main>
    </div>
  </div>
</div>
