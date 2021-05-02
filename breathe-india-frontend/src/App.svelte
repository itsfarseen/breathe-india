<script>
  import { Router, navigate, Link, link, Route } from "svelte-routing";
  import NavLink from "./components/NavLink.svelte";
  import E404 from "./components/E404.svelte";
  import Me from "./components/Me.svelte";
  import Post from "./components/Post.svelte";
  import Posts from "./components/Posts.svelte";
  import PostEdit from "./components/PostEdit.svelte";
  import PostDelete from "./components/PostDelete.svelte";

  let linkClass =
    "flex-1 text-center p-3 border-b-4 uppercase text-sm font-semibold border-transparent hover:border-white";

  let activeTabClass = "border-gray-200";

  let jwt = localStorage.getItem("breathe_india_jwt");
  let userid = localStorage.getItem("breathe_india_userid");

  function onLogin(e) {
    jwt = e.detail.token;
    userid = e.detail.userid;
    localStorage.setItem("breathe_india_jwt", jwt);
    localStorage.setItem("breathe_india_userid", userid);
  }

  function onLogout() {
    localStorage.removeItem("breathe_india_jwt");
    localStorage.removeItem("breathe_india_userid");
    jwt = null;
    userid = null;
  }

  function onError(e) {
    if (e.detail?.name == "HTTPError" && e.detail?.message == "Unauthorized") {
      onTokenExpired();
    } else if (
      e.detail?.name == "TypeError" &&
      e.detail.message.startsWith("NetworkError")
    ) {
      onNetworkError();
    } else {
      onUnexpectedError(e.detail);
    }
  }

  let toastMsg = "";
  let toastClass = "";
  function toast(type, message) {
    toastMsg = message;
    switch (type) {
      case "warn":
        toastClass = "bg-yellow-500";
        break;
      case "error":
        toastClass = "bg-red-500";
        break;
      case "info":
        toastClass = "bg-blue-500";
        break;
    }
  }
  function toastClose() {
    toastMsg = "";
  }

  function onTokenExpired() {
    onLogout();
    toast("info", "Please sign in to continue");
    navigate("/me");
  }

  function onNetworkError() {
    toast(
      "warn",
      "We are detecting connecivity issues. Please check your internet and refresh the app."
    );
  }

  function onUnexpectedError(err) {
    toast(
      "error",
      "We encountered an unexpected error. Please reload the app."
    );
    console.log(err);
  }
</script>

{#if toastMsg}
  <div
    class="absolute text-white text-sm {toastClass} px-4 py-2 leading-tight shadow-md rounded-md inset-x-4 top-32 z-50"
    on:click={toastClose}
  >
    {toastMsg}
  </div>
{/if}

<Router>
  <div class="flex flex-col h-screen">
    <header class="bg-green-800 text-white flex flex-col shadow-md z-10">
      <div class="p-4 pb-2 flex">
        <h1 class="text-xl font-bold">Breathe India</h1>
      </div>
      <div class="flex">
        <NavLink klass={linkClass} activeKlass={activeTabClass} to="/needs">
          Needs
        </NavLink>
        <NavLink klass={linkClass} activeKlass={activeTabClass} to="/supplies">
          Supplies
        </NavLink>
        <NavLink klass={linkClass} activeKlass={activeTabClass} to="/me">
          Me
        </NavLink>
      </div>
    </header>
    <main class="flex-1 flex flex-col overflow-y-scroll">
      <Route path="/needs">
        <Posts type="Needs" on:error={onError} />
      </Route>
      <Route path="/supplies">
        <Posts type="Supplies" on:error={onError} />
      </Route>
      <Route path="/post/:id" let:params>
        <Post post_id={params.id} on:error={onError} {userid} />
      </Route>
      <Route path="/post/:id/update" let:params>
        <PostEdit post_id={params.id} token={jwt} on:error={onError} />
      </Route>
      <Route path="/post/:id/delete" let:params>
        <PostDelete post_id={params.id} token={jwt} on:error={onError} />
      </Route>
      <Route path="/post/new/:typ" let:params>
        {#if jwt == null}
          <Me on:login={onLogin} token={jwt} on:error={onError} />
        {:else}
          <PostEdit typ={params.typ} token={jwt} on:error={onError} />
        {/if}
      </Route>
      <Route path="/me">
        <Me on:login={onLogin} token={jwt} on:error={onError} />
      </Route>
      <Route>
        <E404 />
      </Route>
    </main>
  </div>
</Router>

<style>
  :global(html, body, #app) {
    height: 100vh;
  }

  :global(#app) {
    display: flex;
    flex-direction: column;
  }

  :global(#app > *) {
    flex: 1;
  }
</style>
