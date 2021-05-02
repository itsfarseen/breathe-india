<script>
  import PostRow from "./PostRow.svelte";

  import { createEventDispatcher, onMount } from "svelte";
  import TimeAgo from "javascript-time-ago";
  import api from "../api";
  import { navigate } from "svelte-routing";
  import { fwdError, rememberLastMainTab } from "../utils";

  const N = 20;

  export let type = "";
  export let start = 0;
  export let n = N;
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

  function loadNext() {
    start += N;
    load();
  }

  function loadPrev() {
    start -= N;
    load();
  }

  const timeAgo = new TimeAgo("en-US");
</script>

<div class="flex flex-col">
  <div class="flex flex-col p-4 gap-4 bg-gray-100">
    <button class="button" on:click={() => navigate("/post/new/" + type)}
      >New Post</button
    >
    <form on:submit|preventDefault={load} class="flex flex-col gap-1">
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
      <button class="button">Search</button>
    </form>
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
      {:else}
        <h1 class="text-2xl mt-16 text-center font-bold text-gray-400">
          empty.
        </h1>
      {/each}
      <div class="flex p-2 gap-2">
        {#if start > 0}
          <button class="button" on:click={loadPrev}>Prev</button>
        {/if}
        <div class="flex-1" />
        {#if posts.length > 0}
          <button class="button" on:click={loadNext}>Next</button>
        {/if}
      </div>
    {/await}
  </div>
</div>

<style>
</style>
