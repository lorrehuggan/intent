<script lang="ts">
  import Button from "@/components/ui/button/button.svelte";
  import { Archive, Home, Layers, Plus, Wrench } from "@lucide/svelte";
  import { tv } from "tailwind-variants";
  import * as Dialog from "$lib/components/ui/dialog/index.js";

  const styles = tv({
    slots: {
      container: "h-full w-20 py-8 pt-2 pl-4",
      menu: "flex-center size-full flex-col gap-4 rounded-xl py-4",
    },
  });
  const { container, menu } = styles();
</script>

<div class={container()}>
  <div class={menu()}>
    <a href="/">
      <Button size="icon" variant="secondary"><Home size={16} /></Button>
    </a>
    <Dialog.Root>
      <Dialog.Trigger class="Trigger">
        <Button size="icon" variant="secondary"><Plus size={16} /></Button>
      </Dialog.Trigger>
      <Dialog.Content class="rounded-xl">
        <p>kjaskdj</p>
      </Dialog.Content>
    </Dialog.Root>
    <Button size="icon" variant="secondary"><Layers size={16} /></Button>
    <Button size="icon" variant="secondary"><Archive size={16} /></Button>
    <Button size="icon" variant="secondary">
      <a href="/settings">
        <Wrench size={15} />
      </a>
    </Button>
  </div>
</div>
