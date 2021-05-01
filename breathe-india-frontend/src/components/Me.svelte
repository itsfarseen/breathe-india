<script>
  import api from "../api";
  import { fwdError } from "../utils";
  import { createEventDispatcher } from "svelte";
  export let token = null;
  let profile = null;

  const dispatch = createEventDispatcher();

  window.onGoogleSignin = async (googleUser) => {
    let id_token = googleUser.getAuthResponse().id_token;
    let { our_token } = await fwdError(
      dispatch,
      api.login({ token: id_token })
    );
    dispatch("login", { token: our_token });
  };

  $: if (token != null) {
    profile = fwdError(dispatch, api.profile({ token }));
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
</script>

<div class="bg-gray-200 flex flex-col flex-1">
  {#if token == null}
    <h1 class="text-4xl font-bold text-gray-500 mt-16 mb-8 text-center">
      Sign In
    </h1>
    <div
      class="g-signin2"
      data-longtitle="true"
      data-onsuccess="onGoogleSignin"
    />
  {:else}
    <div class="bg-gray-100 p-4 gap-4 flex">
      {#await profile}
        <div class="flex flex-1 gap-4 animate-pulse">
          <div class="w-24 h-24 rounded-full self-start bg-gray-200" />
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
          class="w-24 rounded-full self-start"
        />
        <div class="flex flex-col justify-start leading-tight gap-1">
          <div>
            <div class="text-xl font-semibold">{profile.name}</div>
            <div class="font-medium text-gray-500">{profile.email}</div>
          </div>
          {#if !editBioState}
            <div
              class="text-gray-500 text-sm leading-tight whitespace-pre-wrap"
            >
              {profile.bio}
            </div>
            <button
              class="text-gray-600 rounded px-2 py-1 border border-gray-400 self-start mt-1"
              on:click={editBio}
              >{profile.bio == "" ? "add" : "edit"} bio</button
            >
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
  {/if}
</div>
