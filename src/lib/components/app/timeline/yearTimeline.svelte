<script lang="ts">
  import Button from "@/components/ui/button/button.svelte";
  import type { Habit, YearTimeline } from "@/types/bindings";
  import { convertDateNoTime } from "@/utils";
  import { CheckIcon } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { tv } from "tailwind-variants";

  let { habit }: { habit: Habit } = $props();
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

  const styles = tv({
    slots: {
      container: "",
      header:
        "flex-center text-foreground mb-2 w-full justify-between rounded-lg bg-neutral-200 p-2 px-3 dark:bg-neutral-800 dark:text-neutral-100",
      headerTitle: "text-sm",
      headerDesc: "text-xs",
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

  const { header, headerTitle, headerDesc, week, node } = styles();
</script>

{#snippet tlnode(day: string)}
  <div
    data-date={convertDateNoTime(day)}
    class={node({
      status: "incomplete",
    })}
  ></div>
{/snippet}

{#snippet tlweek(days: Array<string>)}
  <div class={week()}>
    {#each days as day}
      {@render tlnode(day)}
    {/each}
  </div>
{/snippet}

<div class="">
  <div class={header()}>
    <div>
      <p class={headerTitle()}>{habit.title}</p>
      <p class={headerDesc()}>{habit.description}</p>
    </div>
    <div>
      <Button size="icon">
        <CheckIcon size={16} />
      </Button>
    </div>
  </div>
  {#each yearTimeline as weekday}
    {@render tlweek(weekday)}
  {/each}
</div>
