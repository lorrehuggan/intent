<script lang="ts">
  import Badge from "@/components/ui/badge/badge.svelte";
  import Button from "@/components/ui/button/button.svelte";
  import { Plus } from "@lucide/svelte";
  import { tv } from "tailwind-variants";
  import AddHabit from "./addHabit.svelte";

  const styles = tv({
    slots: {
      container: "border-muted-foreground/20 flex-center justify-between border-b-[1px] pb-2",
    },
  });
  const { container } = styles();
</script>

<div class={container()}>
  <pre class="text-xs">All Habits</pre>
  <AddHabit />
</div>
