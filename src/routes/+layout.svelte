<script>
  import Sidebar from "@/components/app/sidebar/sidebar.svelte";
  import { tv } from "tailwind-variants";
  import "../styles/app.css";

  let { children } = $props();

  const styles = tv({
    slots: {
      container: "flex",
      bodyWrapper: "p-8",
    },
  });

  const { container, bodyWrapper } = styles();
</script>

<div class={container()}>
  <Sidebar />
  <div class={bodyWrapper()}>
    {@render children()}
  </div>
</div>
