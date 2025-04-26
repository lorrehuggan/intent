<script lang="ts">
  import Switch from "@/components/ui/switch/switch.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { tv } from "tailwind-variants";

  const html = document.querySelector("html");
  let { data } = $props();

  let darkModeToggle = $state(false);
  let highlightCurrentDayToggle = $state(false);

  onMount(() => {
    if (!html) return;
    const theme = html.getAttribute("data-theme");
    const highlightCurrentDay = html.getAttribute("data-highlight-current-day");

    highlightCurrentDay === "true"
      ? (highlightCurrentDayToggle = true)
      : (highlightCurrentDayToggle = false);

    theme === "dark" ? (darkModeToggle = true) : (darkModeToggle = false);
  });

  async function toggleDarkMode() {
    if (!html) return;
    const theme = html.getAttribute("data-theme");
    html.setAttribute("data-theme", theme === "dark" ? "light" : "dark");
    html.classList.toggle("dark");
    invoke("set_theme", { theme: theme === "dark" ? "light" : "dark" });
  }

  async function toggleHighlightCurrentDay() {
    if (!html) return;
    const highlightCurrentDay = html.getAttribute("data-highlight-current-day");
    html.setAttribute(
      "data-highlight-current-day",
      highlightCurrentDay === "true" ? "false" : "true",
    );
    invoke("set_highlight_current_day", {
      highlightCurrentDay: highlightCurrentDay === "true" ? false : true,
    });
  }

  const styles = tv({
    slots: {
      header: "page-heading",
      settingContainer: "mx-auto mt-8 w-[600px] justify-between gap-4 pb-4",
      settingList:
        "bg-muted-foreground/10 mt-2 space-y-2 rounded-lg border-[1px] border-black/20 p-4 dark:border-white/20",
      settingBlock:
        "flex-center justify-between border-b-[1px] border-black/20 pb-2 last:border-0 last:pb-0 dark:border-white/20",
      settingCategory: "text-lg",
      settingTitle: "text-sm",
      settingDescription: "text-muted-foreground text-xs",
    },
  });

  const {
    header,
    settingTitle,
    settingCategory,
    settingBlock,
    settingContainer,
    settingDescription,
    settingList,
  } = styles();
</script>

<h1 class={header()}>Settings</h1>
<!-- Appearance -->
<div class={settingContainer()}>
  <h4 class={settingCategory()}>Appearance</h4>
  <div class={settingList()}>
    <div class={settingBlock()}>
      <div class="">
        <p class={settingTitle()}>Mode</p>
        <p class={settingDescription()}>
          {`Set theme to ${darkModeToggle ? "light" : "dark"} mode`}
        </p>
      </div>
      <Switch onCheckedChange={toggleDarkMode} bind:checked={darkModeToggle} />
    </div>
    <div class={settingBlock()}>
      <div class="">
        <p class={settingTitle()}>Highlight</p>
        <p class={settingDescription()}>Highlight the current day</p>
      </div>
      <Switch
        onCheckedChange={toggleHighlightCurrentDay}
        bind:checked={highlightCurrentDayToggle}
      />
    </div>
  </div>
</div>
