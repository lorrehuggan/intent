<script>
  import Button from "@/components/ui/button/button.svelte";
  import { Archive, Layers, Plus, Wrench } from "@lucide/svelte";
  import { tv } from "tailwind-variants";

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
    <Button size="icon" variant="secondary"><Plus size={15} /></Button>
    <Button size="icon" variant="secondary"><Layers size={15} /></Button>
    <Button size="icon" variant="secondary"><Archive size={15} /></Button>
    <Button size="icon" variant="secondary"><Wrench size={15} /></Button>
  </div>
</div>
