<script>
  import { navigate } from "svelte-routing";
  export let type = "";
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
        />
      </label>
      <label class="field">
        <span>Location</span>
        <input
          class="input"
          placeholder={type === "Needs"
            ? "Where is it needed?"
            : "Where is it available?"}
        />
      </label>
    </div>
  </div>
</div>

<style>
</style>
