<script lang="ts">
  import Button from "@/components/ui/button/button.svelte";
  import { Minus, Square, X } from "@lucide/svelte";
  import { tv } from "tailwind-variants";
  import { Window } from "@tauri-apps/api/window";
  const appWindow = new Window("main");

  const styles = tv({
    slots: {
      container: "flex-center bg-background h-10 w-screen justify-between gap-3 px-2",
      actions: "flex-center",
      close: "hover:bg-red-400",
    },
  });

  const { container, actions, close } = styles();

  function closeWindow() {
    appWindow.close();
  }
  function minimizeWindow() {
    appWindow.minimize();
  }
  function maximizeWindow() {
    appWindow.maximize();
  }
</script>

<nav data-tauri-drag-region class={container()}>
  <div>
    <div class="text-muted-foreground pl-3 text-xs tracking-widest uppercase">Intent</div>
  </div>
  <div class={actions()}>
    <Button onclick={minimizeWindow} size="icon" variant="ghost"><Minus size="15" /></Button>
    <Button onclick={maximizeWindow} size="icon" variant="ghost"><Square size="15" /></Button>
    <Button class={close()} onclick={closeWindow} size="icon" variant="ghost"
      ><X size="15" /></Button
    >
  </div>
</nav>
