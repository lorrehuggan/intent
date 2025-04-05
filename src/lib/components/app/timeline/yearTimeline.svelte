<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { YearTimeline } from "@/types/bindings";
  import { tv } from "tailwind-variants";

  let yearTimeline: Array<Array<string>> = $state([]);

  onMount(async () => {
    const year: YearTimeline = await invoke("create_year_timeline");
    const nodes = year.nodes;
    if (
      !nodes.Sun ||
      !nodes.Mon ||
      !nodes.Tue ||
      !nodes.Wed ||
      !nodes.Thu ||
      !nodes.Fri ||
      !nodes.Sat
    )
      return;
    yearTimeline.push(nodes.Sun);
    yearTimeline.push(nodes.Mon);
    yearTimeline.push(nodes.Tue);
    yearTimeline.push(nodes.Wed);
    yearTimeline.push(nodes.Thu);
    yearTimeline.push(nodes.Fri);
    yearTimeline.push(nodes.Sat);
  });
  function convertDate(node: string) {
    return node.split("T")[0];
  }
  const styles = tv({
    slots: {
      container: "",
      header: "mb-2 h-8 w-full rounded bg-neutral-600 p-2",
      week: "flex-center mb-[2px] gap-[2px]",
      node: "size-[13px] rounded",
    },
    variants: {
      status: {
        completed: {
          node: "bg-rose-400",
        },
        incomplete: {
          node: "bg-emerald-300",
        },
      },
    },
  });

  const { header, week, node } = styles();
</script>

{#snippet tlnode(day: string, test: boolean)}
  {@const x = test ? "incomplete" : "completed"}
  <div
    data-date={convertDate(day)}
    class={node({
      status: x,
    })}
  ></div>
{/snippet}

{#snippet tlweek(days: Array<string>)}
  <div class={week()}>
    {#each days as day, i}
      {@const test = i < 1}
      {@render tlnode(day, test)}
    {/each}
  </div>
{/snippet}

<div class="">
  <div class={header()}></div>
  {#each yearTimeline as weekday}
    {@render tlweek(weekday)}
  {/each}
</div>
