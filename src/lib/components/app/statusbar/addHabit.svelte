<script lang="ts">
  import { tv } from "tailwind-variants";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "@/components/ui/button";
  import { type } from "arktype";
  import { defaults, superForm, type Infer } from "sveltekit-superforms";
  import { arktype } from "sveltekit-superforms/adapters";

  let defaultForm = {
    title: "",
    description: "",
    streak: "",
    category: "",
  };

  const schema = type({
    title: "2 <= string <= 64",
    description: "2 <= string <= 128",
    streak: "string",
    category: "2<= string <= 32",
  });

  const data = defaults(arktype(schema, { defaults: defaultForm }));

  const { form, errors, enhance } = superForm<
    Infer<typeof schema>,
    { status: number; text: string }
  >(data, {
    SPA: true,
    resetForm: false,
    clearOnSubmit: "errors-and-message",
    validators: arktype(schema, { defaults: defaultForm }),
    onUpdate: async ({ form }) => {
      console.log(form);
    },
  });

  const styles = tv({
    slots: {
      errorMessage: "text-xs text-rose-400 capitalize",
      input: "text-muted-foreground border-transparent p-0 shadow-transparent focus-visible:ring-0",
    },
  });
  const { errorMessage, input } = styles();
</script>

<Dialog.Root>
  <Dialog.Trigger>
    <Button class="text-foreground text-xs" variant="default" size="sm">Create Habit</Button>
  </Dialog.Trigger>
  <Dialog.Content>
    <p class="text-muted-foreground text-sm">Create Habit</p>
    <form method="POST" use:enhance class="">
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
      <div class="flex-center gap-2"></div>
      <div class="flex-center mt-2 justify-end gap-2">
        <Dialog.Close>
          <Button size="sm" variant="secondary">Cancel</Button>
        </Dialog.Close>
        <Button type="submit" size="sm">Create</Button>
      </div>
    </form>
  </Dialog.Content>
</Dialog.Root>
