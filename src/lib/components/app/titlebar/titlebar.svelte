<script lang="ts">
  import Button from "@/components/ui/button/button.svelte";
  import { Cog, Minus, Search, Settings, Square, X } from "@lucide/svelte";
  import { tv } from "tailwind-variants";
  import { Window } from "@tauri-apps/api/window";

  const appWindow = new Window("main");

  function closeWindow() {
    appWindow.close();
  }
  function minimizeWindow() {
    appWindow.minimize();
  }
  function maximizeWindow() {
    appWindow.maximize();
  }

  const styles = tv({
    slots: {
      container: "flex-center bg-background h-10 w-screen justify-between gap-3 px-2",
      actions: "flex-center",
      close: "hover:bg-red-400",
      title: "text-muted-foreground pl-3 text-xs tracking-widest uppercase",
    },
  });

  const { container, actions, close, title } = styles();
</script>

<nav data-tauri-drag-region class={container()}>
  <div>
    <div class={title()}>Intent</div>
  </div>
  <div class={actions()}>
    <div class="flex-center mr-4 gap-2">
      <a href="/"><Search size="16" /></a>
      <a href="/settings">
        <Settings size="16" />
      </a>
    </div>
    <Button onclick={minimizeWindow} size="icon" variant="ghost"><Minus size="16" /></Button>
    <Button onclick={maximizeWindow} size="icon" variant="ghost"><Square size="16" /></Button>
    <Button class={close()} onclick={closeWindow} size="icon" variant="ghost"
      ><X size="16" /></Button
    >
  </div>
</nav>
