<script>
  import { createEventDispatcher, onMount } from "svelte";
  import { fwdError, goBackToLastMainTab } from "../utils";
  import api from "../api";
  import TimeAgo from "javascript-time-ago";
  import { navigate } from "svelte-routing";

  const timeAgo = new TimeAgo("en-US");

  export let post_id = "";
  export let token = "";

  let root;

  const dispatch = createEventDispatcher();

  async function doDelete() {
    await api.deletePost({ id: post_id, token });
    goBackToLastMainTab();
  }

  onMount(() => {
    root.parentNode.scrollTop = 0;
  });
</script>

<div
  class="flex flex-col bg-gray-100 p-4 gap-2 min-h-screen justify-start"
  bind:this={root}
>
  <h1 class="text-2xl font-bold text-gray-500">Are you sure to delete?</h1>
  <p class="text-gray-500">This action cannot be undone.</p>
  <button class="button-danger" on:click={doDelete}>Yes, Delete</button>
  <button class="button-neutral" on:click={() => window.history.back()}
    >No, Cancel</button
  >
</div>

<style>
</style>
