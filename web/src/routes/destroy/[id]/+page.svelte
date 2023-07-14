<script lang="ts">
  import type { ResponseBody } from '$lib/@types';

  export let data: ResponseBody;
</script>

<div>{JSON.stringify(data)}</div>
