<script lang="ts">
  import Button from "@/components/ui/button/button.svelte";
  import { Archive, Home, Layers, Plus, Wrench } from "@lucide/svelte";
  import { tv } from "tailwind-variants";
  import * as Dialog from "$lib/components/ui/dialog/index.js";

  const styles = tv({
    slots: {
      container: "h-full px-4 py-8 pt-2",
      menu: "flex-center size-full flex-col gap-4 rounded-xl py-4",
    },
  });
  const { container, menu } = styles();
</script>

<div class={container()}>
  <div class={menu()}>
    <a href="/">
      <Button size="icon" variant="ghost"><Home size={16} /></Button>
    </a>
    <Button size="icon" variant="ghost"><Layers size={14} /></Button>
    <Button size="icon" variant="ghost"><Archive size={14} /></Button>
  </div>
</div>
