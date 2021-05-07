<script>
  import { createEventDispatcher } from "svelte";
  import { fwdError } from "../utils";
  import api from "../api";
  import TimeAgo from "javascript-time-ago";
  import { navigate } from "svelte-routing";

  const timeAgo = new TimeAgo("en-US");

  export let post_id = "";
  export let userid = "";

  const dispatch = createEventDispatcher();

  let post = fwdError(dispatch, api.getPostSingle({ id: post_id }));
</script>

<div class="flex flex-col bg-gray-100 p-4 gap-2 flex-1 justify-start">
  {#await post}
    Loading
  {:then res}
    {#if res.user}
      <div class="bg-gray-50 -m-4 mb-1 p-4 flex gap-4 ">
        <img
          src={res.user.profile_pic_url}
          alt="Profile"
          class="w-16 rounded-full self-start"
        />
        <div class="flex flex-col justify-start leading-tight gap-1">
          <div class="flex items-center gap-2">
            <div class="text-lg font-semibold">{res.user.name}</div>
            {#if res.user.verified}
              <img src="/verified.png" alt="Verified badge" class="w-4 h-4" />
            {/if}
          </div>
          <div class="text-gray-500 text-sm leading-tight whitespace-pre-wrap">
            {res.user.bio}
          </div>
        </div>
      </div>
    {/if}
    <div class="text-gray-500 text-xs italics">
      Sl. No: <span class="uppercase">{res.post.id.slice(0, 6)}</span>
    </div>
    <h1 class="text-2xl font-bold text-gray-500">
      {#if res.post.post_type == "Needs"}
        what is needed ..
      {:else if res.post.post_type == "Supplies"}
        what is available ..
      {/if}
    </h1>
    <label class="field">
      <span>Item</span>
      <input
        class="input"
        size="1"
        readonly
        placeholder="N/A"
        value={res.post.item}
      />
    </label>
    <label class="field">
      <span>Quantity</span>
      <input
        class="input"
        size="1"
        readonly
        placeholder="N/A"
        value={res.post.quantity}
      />
    </label>
    <h1 class="text-2xl font-bold text-gray-500">
      {#if res.post.post_type == "Needs"}
        at place ..
      {:else if res.post.post_type == "Supplies"}
        from place ..
      {/if}
    </h1>
    <label class="field">
      <span>State/UT</span>
      <input
        class="input"
        size="1"
        readonly
        placeholder="N/A"
        value={res.post.state}
      />
    </label>
    <label class="field">
      <span>District</span>
      <input
        class="input"
        size="1"
        readonly
        placeholder="N/A"
        value={res.post.district}
      />
    </label>
    <label class="field">
      <span>City</span>
      <input
        class="input"
        size="1"
        readonly
        placeholder="N/A"
        value={res.post.city}
      />
    </label>
    <label class="field">
      <span>Spot</span>
      <input
        class="input"
        size="1"
        readonly
        placeholder="N/A"
        value={res.post.spot}
      />
    </label>
    <h1 class="text-2xl font-bold text-gray-500">more info ..</h1>
    <div class="input" style="min-height: 6em;">{res.post.message}</div>
    <label class="field">
      <span>Posted</span>
      <input
        class="input"
        size="1"
        readonly
        value={new Date(res.post.created_at).toLocaleString()}
      />
    </label>
    <label class="field">
      <span>Updated</span>
      <input
        class="input"
        size="1"
        readonly
        value={new Date(res.post.updated_at).toLocaleString()}
      />
    </label>
    {#if userid == res.post.userid}
      <button
        class="button"
        on:click={() => navigate("/post/" + res.post.id + "/update")}
        >Update</button
      >
      <button
        class="button-danger"
        on:click={() => navigate("/post/" + res.post.id + "/delete")}
        >Delete</button
      >
    {/if}
  {/await}
</div>

<style>
</style>
