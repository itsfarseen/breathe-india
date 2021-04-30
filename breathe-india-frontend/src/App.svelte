<script>
  import { Router, navigate, Link, link, Route } from "svelte-routing";
  import NavLink from "./components/NavLink.svelte";
  import E404 from "./components/E404.svelte";
  import Me from "./components/Me.svelte";
  import Post from "./components/Post.svelte";
  import Posts from "./components/Posts.svelte";

  let activeTabClass = "border-gray-200";
  let inactiveTabClass = "border-transparent hover:border-white";
</script>

<Router>
  <div class="flex flex-col">
    <header class="bg-green-800 text-white flex flex-col shadow-md">
      <div class="p-4 pb-2 flex">
        <h1 class="text-xl font-bold">Breathe India</h1>
      </div>
      <div class="flex">
        <NavLink
          klass="flex-1 text-center p-3 border-b-4 uppercase text-sm font-semibold"
          activeKlass={activeTabClass}
          inactiveKlass={inactiveTabClass}
          to="/needs"
        >
          Needs
        </NavLink>
        <NavLink
          klass="flex-1 text-center p-3 border-b-4 uppercase text-sm font-semibold"
          activeKlass={activeTabClass}
          inactiveKlass={inactiveTabClass}
          to="/supplies"
        >
          Supplies
        </NavLink>
        <NavLink
          klass="flex-1 text-center p-3 border-b-4 uppercase text-sm font-semibold"
          activeKlass={activeTabClass}
          inactiveKlass={inactiveTabClass}
          to="/me"
        >
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
      <Route>
        <E404 />
      </Route>
    </main>
  </div>
</Router>

<style>
  header {
    /* background: #00596c; */
  }

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
