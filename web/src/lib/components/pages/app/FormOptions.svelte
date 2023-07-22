<script lang="ts">
  import { page } from '$app/stores';
  import type { ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';
  import Checkbox from '$lib/components/elements/Checkbox.svelte';
  import Dropdown from '$lib/components/elements/Dropdown.svelte';
  import Input from '$lib/components/elements/Input.svelte';

  let form: ResponseBody = $page.form;
</script>

<section id="options" class="relative mt-4 hidden target:block">
  <h3 class="mb-1 text-xl font-semibold">Note self-destructs</h3>

  <div class="flex flex-col justify-between space-y-2.5 md:flex-row md:space-x-5 md:space-y-0">
    <Dropdown
      {form}
      name="duration_hours"
      options={[
        { text: 'after reading it', value: '0' },
        { text: '1 hour from now', value: '1' },
        { text: '24 hours from now', value: '24' },
        { text: '7 days from now', value: '168' },
        { text: '30 days from now', value: '720' }
      ]}
    />
    <Checkbox {form} name="destroy_without_confirmation" />
  </div>

  <h3 class="mb-1 text-xl font-semibold">Manual password</h3>

  <div class="flex flex-col justify-between space-y-2.5 md:flex-row md:space-x-5 md:space-y-0">
    <Input {form} type="password" name="manual_password" label="Enter a custom password to encrypt the note" />
    <Input {form} type="password" name="manual_password_confirm" label="Confirm password" />
  </div>

  <h3 class="mb-1 text-xl font-semibold">Destruction notification</h3>

  <div class="flex flex-col justify-between space-y-2.5 md:flex-row md:space-x-5 md:space-y-0">
    <Input {form} name="notify_email" label="E-mail to notify when note is destroyed" />
  </div>

  <div class="absolute -bottom-14 right-1/2">
    <Button {form} title="close" href="#" icon="i-line-md:close-circle" />
  </div>
</section>
