<script>
  import Switch from "@/components/ui/switch/switch.svelte";
  import { getUserSettings } from "@/context/user/settings.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  const html = document.querySelector("html");
  const userSettings = getUserSettings();
  let switchToggle = $state(false);

  onMount(() => {
    if (!html) return;
    if (html.classList.contains("dark")) {
      switchToggle = true;
    } else {
      switchToggle = false;
    }
  });

  async function toggleTheme() {
    if (!userSettings || !html) return;
    const classList = html.classList;
    if (classList.contains("dark")) {
      classList.remove("dark");
      switchToggle = false;
      try {
        await invoke("set_user_settings", { payload: { ...userSettings, theme: "light" } });
      } catch (e) {
        console.log(e);
      }
    } else {
      classList.add("dark");
      switchToggle = true;
      try {
        await invoke("set_user_settings", { payload: { ...userSettings, theme: "dark" } });
      } catch (e) {
        console.log(e);
      }
    }
  }
</script>

<h1 class="text-3xl">Settings</h1>
<a href="/">home</a>
<div class="flex-center gap-4">
  <p>{`Set theme to ${switchToggle ? "light" : "dark"} mode`}</p>
  <Switch onCheckedChange={toggleTheme} bind:checked={switchToggle} />
</div>
