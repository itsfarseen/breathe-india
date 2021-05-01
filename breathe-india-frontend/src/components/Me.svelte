<script>
  import api from "../api";
  import { createEventDispatcher } from "svelte";
  export let token = null;

  const dispatch = createEventDispatcher();

  window.onGoogleSignin = async (googleUser) => {
    let id_token = googleUser.getAuthResponse().id_token;
    let { our_token } = await api.login({ token: id_token });
    console.log(our_token);
    dispatch("login", { token: our_token });
  };
</script>

<div class="bg-gray-200 flex flex-col flex-1 items-center pt-16 gap-8">
  {#if token == null}
    <h1 class="text-4xl font-bold text-gray-500">Sign In</h1>
    <div
      class="g-signin2"
      data-longtitle="true"
      data-onsuccess="onGoogleSignin"
    />
  {/if}
</div>
