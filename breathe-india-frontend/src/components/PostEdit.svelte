<script>
  import api from "../api";
  import { navigate } from "svelte-routing";
  import { createEventDispatcher } from "svelte";
  import { fwdError } from "../utils";
  import { prevent_default } from "svelte/internal";

  export let post_id = null;
  export let typ = null;
  export let token;

  const dispatch = createEventDispatcher();

  let item = "";
  let quantity = "";
  let state = "";
  let district = "";
  let city = "";
  let spot = "";
  let message = "";
  let form;

  async function load() {
    if (post_id != null) {
      let { post } = await api.getPostSingle({ id: post_id });
      typ = post.post_type;
      item = post.item;
      quantity = post.quantity;
      state = post.state;
      district = post.district;
      city = post.city;
      spot = post.spot;
      message = post.message;
    }
  }
  load();

  let saving = false;
  async function save() {
    let post = {
      post_type: typ,
      state,
      district,
      city,
      spot,
      message,
      item,
      quantity,
    };

    saving = true;

    if (post_id) {
      try {
        let post_saved = await fwdError(
          dispatch,
          api.updatePost({ ...post, id: post_id, token })
        );
        navigate("/post/" + post_saved.id);
      } catch (err) {}
    } else {
      try {
        let post_saved = await fwdError(
          dispatch,
          api.createPost({ ...post, token })
        );
        navigate("/post/" + post_saved.id);
      } catch (err) {}
    }
    saving = false;
  }

  function cancel() {
    window.history.back();
  }
</script>

<form
  on:submit|preventDefault={save}
  bind:this={form}
  class="flex flex-col bg-gray-100 p-4 gap-2 min-h-screen justify-start"
>
  <h1 class="text-2xl font-bold text-gray-500">
    {#if typ == "Needs"}
      I need ..
    {:else if typ == "Supplies"}
      I can supply ..
    {/if}
  </h1>

  <label class="field">
    <span>Item</span>
    <input
      class="input"
      size="1"
      required
      placeholder="Oxygen"
      bind:value={item}
    />
  </label>
  <label class="field">
    <span>Quantity</span>
    <input
      class="input"
      size="1"
      required
      placeholder="20L"
      bind:value={quantity}
    />
  </label>
  <h1 class="text-2xl font-bold text-gray-500">
    {#if typ == "Needs"}
      at place ..
    {:else if typ == "Supplies"}
      from place ..
    {/if}
  </h1>
  <label class="field">
    <span>State/UT</span>
    <input
      class="input"
      size="1"
      required
      placeholder="Kerala"
      bind:value={state}
    />
  </label>
  <label class="field">
    <span>District</span>
    <input
      class="input"
      size="1"
      required
      placeholder="Kannur"
      bind:value={district}
    />
  </label>
  <label class="field">
    <span>City</span>
    <input
      class="input"
      size="1"
      required
      placeholder="Anjarakkandy"
      bind:value={city}
    />
  </label>
  <label class="field">
    <span>Spot</span>
    <input
      class="input"
      size="1"
      required
      placeholder="Kannur Govt. Medical College"
      bind:value={spot}
    />
  </label>
  <h1 class="text-2xl font-bold text-gray-500">more info ..</h1>
  <textarea
    class="input"
    placeholder="How to contact you, anything else to note, etc"
    bind:value={message}
  />
  <button class="button" disabled={saving}>
    {#if post_id == null}
      Save
    {:else}
      Update
    {/if}
  </button>
  <button class="button-neutral" type="reset" on:click={cancel}>Cancel</button>
</form>

<style>
  textarea {
    @apply border border-gray-200 p-1 px-2;
  }
</style>
