<script>
  import { Router, navigate, Link, link, Route } from "svelte-routing";
  import NavLink from "./components/NavLink.svelte";
  let activeTabClass = "border-white";
  let inactiveTabClass = "border-transparent hover:border-green-500";
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
      </div>
    </header>
    <Route path="/needs">
      <main class="flex-1">needs</main>
    </Route>
    <Route path="/supplies">
      <main class="flex-1">supplies</main>
    </Route>
  </div>
</Router>

<style>
  header {
    /* background: #00596c; */
  }
</style>
