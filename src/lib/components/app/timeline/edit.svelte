<script lang="ts">
  import { tv } from "tailwind-variants";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Button } from "@/components/ui/button";
  import { queryClient } from "@/context/query";
  import type { Habit } from "@/types/bindings";
  import { Box, CalendarFold, Clock, PaintRoller, Trash } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { type } from "arktype";
  import clsx from "clsx";
  import SuperDebug, { defaults, superForm, type Infer } from "sveltekit-superforms";
  import { arktype } from "sveltekit-superforms/adapters";

  let { habit }: { habit: Habit } = $props();
  let editOpen = $state(false);
  let deleteOpen = $state(false);

  async function openChange(open: boolean) {
    if (!open) {
      queryClient.invalidateQueries({ queryKey: ["all_habits"] });
    }
  }

  async function deleteHabit() {
    try {
      const response = await invoke("delete_habit", { id: habit.id });
      if (response === "ok") {
        deleteOpen = false;
        editOpen = false;
        queryClient.invalidateQueries({ queryKey: ["all_habits"] });
      }
    } catch (e) {
      console.log(e);
    }
  }

  let defaultForm = {
    title: habit.title,
    description: habit.description,
    streak: habit.streak,
    category: habit.category,
    color: habit.theme,
    reminder: habit.reminder,
  };

  const schema = type({
    title: "2 <= string <= 32",
    description: "2 <= string <= 64",
    streak: "string",
    category: "string",
    color: "string",
    reminder: "string | null",
  });

  const data = defaults(arktype(schema, { defaults: defaultForm }));

  const { form, errors, enhance } = superForm<
    Infer<typeof schema>,
    { status: number; text: string }
  >(data, {
    id: String(habit.id),
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors-and-message",
    validators: arktype(schema, { defaults: defaultForm }),
    onChange: async ({ get }) => {
      const title = get("title");
      const description = get("description");
      const category = get("category");
      const reminder = get("reminder");
      const streak = get("streak");
      const theme = get("color");

      const form = {
        id: habit.id,
        completions: 0,
        created: habit.created,
        status: "onGoing",
        updated: new Date().toISOString(),
        title,
        description,
        category,
        reminder,
        streak,
        theme,
      };
      try {
        const response = await invoke("update_habit", { habit: form });
        if (response === "ok") {
        }
        return;
      } catch (e) {
        console.error(e);
      }
    },
    onUpdate: async ({ form }) => {
      console.log("update");
    },
  });

  const styles = tv({
    slots: {
      errorMessage: "text-xs text-rose-400 capitalize",
      input:
        "text-muted-foreground placeholder:text-muted-foreground/60 border-transparent p-0 shadow-transparent focus-visible:ring-0",
      selectTrigger:
        "text-muted-foreground flex-center bg-muted-foreground/5 hover:bg-muted-foreground/10 color-fade h-8 w-auto justify-start gap-1.5 p-1.5 text-[11px] capitalize shadow-none focus:ring-0 focus-visible:ring-0",
      destructive: "border-[1px] border-red-400 bg-red-300 text-xs text-red-800 hover:text-white",
    },
  });

  const { errorMessage, input, selectTrigger, destructive } = styles();

  const streaks = [
    { value: "daily", label: "Daily" },
    { value: "weekly", label: "Weekly" },
    { value: "monthly", label: "Monthly" },
  ];

  const categories = [
    { value: "health", label: "Health" },
    { value: "fitness", label: "Fitness" },
    { value: "nutrition", label: "Nutrition" },
    { value: "leisure", label: "Leisure" },
    { value: "productivity", label: "Productivity" },
    { value: "finace", label: "Finace" },
    { value: "personalGrowth", label: "Personal Growth" },
    { value: "social", label: "Social" },
    { value: "creative", label: "Creative" },
    { value: "study", label: "Study" },
    { value: "home", label: "Home" },
    { value: "work", label: "Work" },
    { value: "school", label: "School" },
    { value: "morning", label: "Morning" },
    { value: "afternoon", label: "Afternoon" },
    { value: "evening", label: "Evening" },
    { value: "night", label: "Night" },
    { value: "other", label: "Other" },
  ];

  const habitColors = [
    { value: "red", label: "Red", class: "bg-red-400" },
    { value: "green", label: "Green", class: "bg-emerald-400" },
    { value: "blue", label: "Blue", class: "bg-blue-400" },
    { value: "rose", label: "Rose", class: "bg-rose-400" },
    { value: "mint", label: "Mint", class: "bg-lime-400" },
    { value: "sky", label: "Sky", class: "bg-sky-400" },
    { value: "amber", label: "Amber", class: "bg-amber-400" },
    { value: "indigo", label: "Indigo", class: "bg-indigo-400" },
    { value: "neutral", label: "Neutral", class: "bg-neutral-400" },
  ];

  const reminderDays = [
    { value: "mon", label: "Mon" },
    { value: "tue", label: "Tue" },
    { value: "wed", label: "Wed" },
    { value: "thu", label: "Thu" },
    { value: "fri", label: "Fri" },
    { value: "sat", label: "Sat" },
    { value: "sun", label: "Sun" },
    { value: "no reminder", label: "No Reminder" },
  ];
</script>

<Dialog.Root
  onOpenChange={openChange}
  bind:open={() => editOpen, (newOpen) => (editOpen = newOpen)}
>
  <Dialog.Trigger>{habit.title}</Dialog.Trigger>
  <Dialog.Content>
    <form method="POST" use:enhance>
      <div>
        <Input
          bind:value={$form.title}
          type="text"
          class={input({
            class: "text-xl placeholder:text-xl md:text-2xl md:placeholder:text-2xl",
          })}
          placeholder="Habit Title"
        />
        {#if $errors.title}
          <span class={errorMessage()}>{$errors.title}</span>
        {/if}
      </div>
      <div>
        <Input
          bind:value={$form.description}
          type="text"
          class={input({
            class: "text-sm placeholder:text-sm",
          })}
          placeholder="Add a short description"
        />
        {#if $errors.description}
          <span class={errorMessage()}>{$errors.description}</span>
        {/if}
      </div>
      <div class="flex-center gap-2">
        <Select.Root type="single" name="streak" bind:value={$form.streak}>
          <Select.Trigger class={selectTrigger()}
            ><CalendarFold size="14" class="text-blue-300" />
            <span>{$form.streak}</span></Select.Trigger
          >
          <Select.Content>
            <Select.Group>
              <Select.GroupHeading class="text-muted-foreground text-xs"
                >Streak Goal</Select.GroupHeading
              >
              {#each streaks as streak}
                <Select.Item
                  class="text-muted-foreground text-xs"
                  value={streak.value}
                  label={streak.label}
                />
              {/each}
            </Select.Group>
          </Select.Content>
        </Select.Root>
        <Select.Root type="single" name="category" bind:value={$form.category}>
          <Select.Trigger class={selectTrigger()}
            ><Box size="14" class="text-orange-500" />
            <span
              >{#if $form.category.length}
                {$form.category}
              {:else}
                Category
              {/if}</span
            ></Select.Trigger
          >
          <Select.Content>
            <Select.Group>
              <Select.GroupHeading class="text-muted-foreground text-xs"
                >Category</Select.GroupHeading
              >
              {#each categories as category}
                <Select.Item
                  class="text-muted-foreground text-xs"
                  value={category.value}
                  label={category.label}
                />
              {/each}
            </Select.Group>
          </Select.Content>
        </Select.Root>
        <Select.Root type="single" name="color" bind:value={$form.color}>
          <Select.Trigger class={selectTrigger()}
            ><PaintRoller size="14" class="text-neutral-700" />
            <span>
              {#if $form.color.length}
                {$form.color}
              {:else}
                Theme
              {/if}
            </span></Select.Trigger
          >
          <Select.Content>
            <Select.Group>
              <Select.GroupHeading class="text-muted-foreground text-xs">Color</Select.GroupHeading>
              {#each habitColors as color}
                <div class="flex-center gap-1">
                  <div class={clsx(`h-2.5 w-2.5 rounded-full ${color.class}`)}></div>
                  <Select.Item
                    class="text-muted-foreground flex-1/2 text-xs"
                    value={color.value}
                    label={color.label}
                  />
                </div>
              {/each}
            </Select.Group>
          </Select.Content>
        </Select.Root>
        <Select.Root type="single" name="color" bind:value={$form.reminder!}>
          <Select.Trigger class={selectTrigger()}
            ><Clock size="14" class="text-pink-400" />
            <span>
              {$form.reminder?.length ? $form.reminder : "No Reminder"}
            </span></Select.Trigger
          >
          <Select.Content>
            <Select.Group>
              <Select.GroupHeading class="text-muted-foreground text-xs">Days</Select.GroupHeading>
              {#each reminderDays as day}
                <Select.Item
                  class="text-muted-foreground flex-1/2 text-xs"
                  value={day.value}
                  label={day.label}
                />
              {/each}
            </Select.Group>
          </Select.Content>
        </Select.Root>
        <Dialog.Root bind:open={() => deleteOpen, (newOpen) => (deleteOpen = newOpen)}>
          <Dialog.Trigger
            ><Button size="sm" variant="destructive" class={destructive()}
              ><Trash size="14" /></Button
            ></Dialog.Trigger
          >
          <Dialog.Content>
            <Dialog.Header>
              <Dialog.Title class="text-red-400">Are you sure absolutely sure?</Dialog.Title>
              <Dialog.Description>
                This action cannot be undone. This will permanently delete this habits data.
              </Dialog.Description>
            </Dialog.Header>
            <div class="mt-2 flex items-center justify-end gap-2">
              <Dialog.Close class="Close">
                <Button size="sm" variant="secondary">Cancel</Button>
              </Dialog.Close>
              <Button size="sm" variant="destructive" class={destructive()} onclick={deleteHabit}
                >Delete!</Button
              >
            </div>
          </Dialog.Content>
        </Dialog.Root>
      </div>
    </form>
  </Dialog.Content>
</Dialog.Root>
