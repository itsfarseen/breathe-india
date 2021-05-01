<script>
  export let type = "";
</script>

<div class="flex flex-col">
  <div class="flex flex-col p-4 gap-4 bg-gray-100">
    <button class="button">New Post</button>
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
  .button {
    @apply bg-green-500 text-white p-1 px-4 rounded-sm shadow-md;
    @apply active:shadow-none;
  }

  .field {
    @apply border border-gray-300 bg-gray-100 rounded bg-clip-border;
    @apply flex items-center;
  }

  .field > span {
    @apply px-2 p-1 w-20 text-gray-600;
    @apply border-r border-gray-300;
  }

  .field > input {
    @apply px-2 p-1 flex-1;
  }
</style>
