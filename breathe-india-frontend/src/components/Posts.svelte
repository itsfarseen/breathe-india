<script>
  import PostRow from "./PostRow.svelte";

  import { createEventDispatcher, onMount } from "svelte";
  import TimeAgo from "javascript-time-ago";
  import api from "../api";
  import { navigate } from "svelte-routing";
  import { fwdError, rememberLastMainTab } from "../utils";

  export let type = "";
  export let start = 0;
  export let n = 100;
  export let item = "";
  export let location = "";

  onMount(rememberLastMainTab);
  const dispatch = createEventDispatcher();

  let posts;

  function load() {
    posts = fwdError(
      dispatch,
      api.getPosts({ typ: type, start, n, item, location })
    );
  }

  load();

  const timeAgo = new TimeAgo("en-US");
</script>

<div class="flex flex-col">
  <div class="flex flex-col p-4 gap-4 bg-gray-100">
    <button class="button" on:click={() => navigate("/post/new/" + type)}
      >New Post</button
    >
    <div class="flex flex-col gap-1">
      <div class="text-sm uppercase font-medium text-gray-500">Filter</div>
      <label class="field">
        <span>Item</span>
        <input
          class="input"
          placeholder={type === "Needs"
            ? "What is needed?"
            : "What is available?"}
          bind:value={item}
        />
      </label>
      <label class="field">
        <span>Location</span>
        <input
          class="input"
          placeholder={type === "Needs"
            ? "Where is it needed?"
            : "Where is it available?"}
          bind:value={location}
        />
      </label>
      <button class="button" on:click={load}>Search</button>
    </div>
  </div>
  <div class="flex-1 flex flex-col divide-y divide-gray-300">
    {#await posts}
      <h1
        class="text-2xl mt-16 text-center font-bold text-gray-500 animate-pulse"
      >
        Loading ..
      </h1>
    {:then posts}
      {#each posts as post}
        <PostRow {post} />
      {/each}
    {/await}
  </div>
</div>

<style>
</style>
