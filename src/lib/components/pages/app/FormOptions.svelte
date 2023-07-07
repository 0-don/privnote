<script>
  import Button from '$lib/components/elements/Button.svelte';
  import Checkbox from '$lib/components/elements/Checkbox.svelte';

  import Dropdown from '$lib/components/elements/Dropdown.svelte';
  import Input from '$lib/components/elements/Input.svelte';
</script>

<section id="options" class="relative mt-4 hidden target:block">
  <h3 class="mb-1 text-xl font-semibold">Note self-destructs</h3>

  <div class="flex justify-between space-x-5">
    <Dropdown
      name="duration_hours"
      options={[
        { text: 'after reading it', value: 0 },
        { text: '1 hour from now', value: 1 },
        { text: '24 hours from now', value: 24 },
        { text: '7 days from now', value: 168 },
        { text: '30 days from now', value: 720 }
      ]}
    />
    <Checkbox name="destroy_without_confirmation" />
  </div>

  <h3 class="mb-1 text-xl font-semibold">Manual password</h3>

  <div class="flex justify-between space-x-5">
    <Input type="password" name="manual_password" label="Enter a custom password to encrypt the note" />
    <Input type="password" name="manual_password_confirm" label="Confirm password" />
  </div>

  <div class="flex justify-end"><Button href="" icon="i-line-md:close-circle" /></div>
</section>
