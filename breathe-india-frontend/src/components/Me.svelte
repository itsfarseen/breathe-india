<script>
  import PostRow from "./PostRow.svelte";
  import api from "../api";
  import { fwdError, rememberLastMainTab } from "../utils";
  import { createEventDispatcher, onMount } from "svelte";
  export let token = null;
  let profile = null;
  let posts = null;
  let needs = null;
  let supplies = null;

  onMount(rememberLastMainTab);

  const dispatch = createEventDispatcher();

  window.onGoogleSignin = async (googleUser) => {
    let id_token = googleUser.getAuthResponse().id_token;
    let { our_token, userid } = await fwdError(
      dispatch,
      api.login({ token: id_token })
    );
    dispatch("login", { token: our_token, userid });
    let auth2 = window.gapi.auth2.getAuthInstance();
    auth2.signOut();
  };

  $: if (token != null) {
    profile = fwdError(dispatch, api.profile({ token }));
    posts = fwdError(dispatch, api.getMyPosts({ token }));
    needs = posts.then((ps) => ps.filter((p) => p.post_type == "Needs"));
    supplies = posts.then((ps) => ps.filter((p) => p.post_type == "Supplies"));
  }

  let editBioState = false;
  let editBioValue = "";
  async function editBio() {
    if (!editBioState) {
      editBioState = true;
      editBioValue = (await profile).bio;
      return;
    }

    profile = fwdError(
      dispatch,
      api.profileUpdate({ bio: editBioValue, token })
    );

    editBioState = false;
  }

  async function logout() {
    dispatch("logout", {});
  }
</script>

<svelte:head>
  {#if token == null}
    <script src="https://apis.google.com/js/platform.js"></script>
    <meta
      name="google-signin-client_id"
      content="138821141331-1rhsperdtvqvog7o93su1hkeh47crv41.apps.googleusercontent.com"
    />
  {/if}
</svelte:head>

<div class="bg-gray-100 flex flex-col flex-1">
  {#if token == null}
    <h1 class="text-4xl font-bold text-gray-500 mt-16 mb-8 text-center">
      Sign In
    </h1>
    <div
      class="g-signin2 self-center"
      data-longtitle="true"
      data-onsuccess="onGoogleSignin"
    />
  {:else}
    <div class="bg-gray-50 p-4 gap-4 flex">
      {#await profile}
        <div class="flex flex-1 gap-4 animate-pulse">
          <div class="w-20 h-20 rounded-full self-start bg-gray-200" />
          <div class="flex-1 flex flex-col justify-start leading-tight gap-2">
            <div class="flex flex-col gap-1">
              <div class="bg-gray-200 h-6" />
              <div class="bg-gray-200 h-4" />
            </div>
            <div class="h-16 bg-gray-200" />
          </div>
        </div>
      {:then profile}
        <img
          src={profile.profile_pic_url}
          alt="Profile"
          class="w-20 rounded-full self-start"
        />
        <div class="flex flex-col justify-start leading-tight gap-1">
          <div>
            <div class="text-lg font-semibold">{profile.name}</div>
            <div class="text-sm font-medium text-gray-500">{profile.email}</div>
          </div>
          {#if !editBioState}
            <div
              class="text-gray-500 text-sm leading-tight whitespace-pre-wrap"
            >
              {profile.bio}
            </div>
            <div class="flex gap-4">
              <button
                class="text-gray-600 rounded px-2 py-1 border border-gray-400 self-start mt-1"
                on:click={editBio}
                >{profile.bio == "" ? "add" : "edit"} bio</button
              >
              <button
                class="text-gray-600 rounded px-2 py-1 border border-gray-400 self-start mt-1"
                on:click={logout}>sign out</button
              >
            </div>
          {:else}
            <textarea
              class="rounded p-2 w-full text-sm leading-tight"
              bind:value={editBioValue}
            />
            <button
              class="text-gray-600 rounded px-2 py-1 border border-gray-400 self-start mt-1"
              on:click={editBio}>save</button
            >
          {/if}
        </div>
      {:catch}
        <span class="text-gray-500 text-center flex-1"
          >Failed to load profile</span
        >
      {/await}
    </div>
    {#await posts}
      <h1
        class="text-2xl mt-16 text-center font-bold text-gray-500 animate-pulse"
      >
        Loading ..
      </h1>
    {:then}
      <div class="flex flex-col">
        <h1 class="text-2xl font-bold text-gray-500 m-4">I need ..</h1>
        {#await needs then posts}
          <div class="flex flex-col divide-y divide-gray-300">
            {#each posts as post}
              <PostRow {post} />
            {:else}
              <h1 class="mb-4 text-2xl text-center font-bold text-gray-400">
                empty.
              </h1>
            {/each}
          </div>
        {/await}
        <h1 class="text-2xl font-bold text-gray-500 m-4">I can supply ..</h1>
        {#await supplies then posts}
          <div class="flex flex-col divide-y divide-gray-300">
            {#each posts as post}
              <PostRow {post} />
            {:else}
              <h1 class="mb-4 text-2xl text-center font-bold text-gray-400">
                empty.
              </h1>
            {/each}
          </div>
        {/await}
      </div>
    {/await}
  {/if}
</div>
