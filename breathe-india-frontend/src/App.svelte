<script>
  import { Router, navigate, Link, link, Route } from "svelte-routing";
  import NavLink from "./components/NavLink.svelte";
  import E404 from "./components/E404.svelte";
  import Me from "./components/Me.svelte";
  import Post from "./components/Post.svelte";
  import Posts from "./components/Posts.svelte";

  let linkClass =
    "flex-1 text-center p-3 border-b-4 uppercase text-sm font-semibold border-transparent hover:border-white";

  let activeTabClass = "border-gray-200";

  let jwt = localStorage.getItem("breathe_india_jwt");

  function onLogin(e) {
    jwt = e.detail.token;
    localStorage.setItem("breathe_india_jwt", jwt);
  }

  function onLogout() {
    localStorage.removeItem("breathe_india_jwt");
    jwt = null;
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
      onUnexpectedError();
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
      case "err":
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
      "We are detecting connecivity issues. Please try reloading if problem persists."
    );
  }

  function onUnexpectedError() {
    toast(
      "error",
      "We encountered an unexpected error. Please reload the app."
    );
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
  <div class="flex flex-col">
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
    <main class="flex-1 flex flex-col">
      <Route path="/needs">
        <Posts type="Needs" />
      </Route>
      <Route path="/supplies">
        <Posts type="Supplies" />
      </Route>
      <Route path="/post/:id" let:params>
        <Post post_id={params.id} />
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
