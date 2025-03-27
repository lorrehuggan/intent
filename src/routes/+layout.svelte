<script>
  import Sidebar from "@/components/app/sidebar/sidebar.svelte";
  import Titlebar from "@/components/app/titlebar/titlebar.svelte";
  import { tv } from "tailwind-variants";
  import "../styles/app.css";

  let { children } = $props();

  const styles = tv({
    slots: {
      windowContainer: "h-screen w-screen overflow-hidden rounded-2xl",
      appContainer: "flex h-[calc(100vh-36px)] overflow-hidden",
      contentContainer: "w-full p-8 pt-2 pl-4",
      mainContainer: "h-full rounded-2xl bg-neutral-100 px-6 py-4 dark:bg-neutral-900",
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
