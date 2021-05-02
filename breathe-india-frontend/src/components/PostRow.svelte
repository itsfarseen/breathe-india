<script>
  import TimeAgo from "javascript-time-ago";
  import { navigate } from "svelte-routing";

  export let post;

  const timeAgo = new TimeAgo("en-US");
</script>

<div
  class="p-4 flex gap-4 active:bg-gray-100"
  on:click={() => navigate("/post/" + post.id)}
>
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

<style>
</style>
