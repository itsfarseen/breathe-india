<script>
  import TimeAgo from "javascript-time-ago";

  import api from "../api";
  import { navigate } from "svelte-routing";
  import Post from "./Post.svelte";

  export let type = "";

  let posts = api.getPosts({ typ: type });

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
  <div class="flex-1 flex flex-col divide-y divide-gray-300">
    {#await posts}
      <h1
        class="text-2xl mt-16 text-center font-bold text-gray-500 animate-pulse"
      >
        Loading ..
      </h1>
    {:then posts}
      {#each posts.posts as post}
        <div class="p-4 flex gap-4 active:bg-gray-100">
          <div class="flex flex-col flex-1">
            <div class="text-lg font-medium text-gray-600">{post.item}</div>
            <div class="text-sm text-gray-600">
              {post.quantity} at {post.spot}
            </div>
            <div class="text-sm text-gray-600">
              {[post.city, post.district, post.state].join(", ")}
            </div>
          </div>
          <div class="text-gray-500">
            {timeAgo.format(new Date(post.updated_at), "mini")}
          </div>
        </div>
      {/each}
    {/await}
  </div>
</div>

<style>
</style>
