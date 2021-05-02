<script>
  import TimeAgo from "javascript-time-ago";
  import { navigate } from "svelte-routing";
  import { identity } from "svelte/internal";

  export let post;

  const timeAgo = new TimeAgo("en-US");
</script>

<div
  class="p-4 flex gap-4 bg-white active:bg-gray-100"
  on:click={() => navigate("/post/" + post.id)}
>
  <div class="flex flex-col flex-1">
    <div class="text-gray-500 text-xs">
      Sl. No: <span class="uppercase">{post.id.slice(0, 6)}</span>
    </div>
    <div class="text-lg font-medium text-gray-600">{post.item}</div>
    <div class="text-sm text-gray-600">
      {post.quantity} at {post.spot}
    </div>
    <div class="text-sm text-gray-600">
      {[post.city, post.district, post.state].join(", ")}
    </div>
  </div>
  <div class="text-gray-500 flex flex-col items-end">
    <div class="flex-1">
      {timeAgo.format(new Date(post.updated_at), "mini")}
    </div>
  </div>
</div>

<style>
</style>
